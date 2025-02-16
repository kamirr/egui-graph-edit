use std::borrow::Cow;

use eframe::egui;
use egui_graph_edit::*;

#[derive(Debug)]
pub struct DummyNodeData;

#[derive(PartialEq, Eq, Debug)]
pub struct DummyDataType;

#[derive(Copy, Clone, Debug, Default)]
pub struct DummyValueType;

#[derive(Clone, Copy)]
pub struct DummyNodeTemplate;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DummyResponse;

type DummyGraphState = ();

impl DataTypeTrait<DummyGraphState> for DummyDataType {
    fn data_type_color(&self, _user_state: &mut DummyGraphState) -> egui::Color32 {
        egui::Color32::from_rgb(238, 207, 60)
    }

    fn name(&self) -> Cow<'_, str> {
        "edge".into()
    }
}

impl NodeTemplateTrait for DummyNodeTemplate {
    type NodeData = DummyNodeData;
    type DataType = DummyDataType;
    type ValueType = DummyValueType;
    type UserState = DummyGraphState;
    type CategoryType = &'static str;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        "Node".into()
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        DummyNodeData
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        graph.add_input_param(
            node_id,
            "in1".to_owned(),
            DummyDataType,
            DummyValueType,
            InputParamKind::ConnectionOnly,
            true,
        );
        graph.add_input_param(
            node_id,
            "in2".to_string(),
            DummyDataType,
            DummyValueType,
            InputParamKind::ConnectionOnly,
            true,
        );
        graph.add_output_param(node_id, "out".to_string(), DummyDataType);
    }
}

pub struct AllMyNodeTemplates;
impl NodeTemplateIter for AllMyNodeTemplates {
    type Item = DummyNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        vec![DummyNodeTemplate]
    }
}

impl WidgetValueTrait for DummyValueType {
    type Response = DummyResponse;
    type UserState = DummyGraphState;
    type NodeData = DummyNodeData;
    fn value_widget(
        &mut self,
        _param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut DummyGraphState,
        _node_data: &DummyNodeData,
    ) -> Vec<DummyResponse> {
        ui.label("x");
        Vec::new()
    }
}

impl UserResponseTrait for DummyResponse {}
impl NodeDataTrait for DummyNodeData {
    type Response = DummyResponse;
    type UserState = DummyGraphState;
    type DataType = DummyDataType;
    type ValueType = DummyValueType;

    fn bottom_ui(
        &self,
        _ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<DummyNodeData, DummyDataType, DummyValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<DummyResponse, DummyNodeData>>
    where
        DummyResponse: UserResponseTrait,
    {
        vec![]
    }
}

type MyEditorState = GraphEditorState<
    DummyNodeData,
    DummyDataType,
    DummyValueType,
    DummyNodeTemplate,
    DummyGraphState,
>;

#[derive(Default)]
pub struct NodeGraphExampleSimple {
    state: MyEditorState,
    user_state: DummyGraphState,
    cached_text_graph_description: String,
}

impl eframe::App for NodeGraphExampleSimple {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_switch(ui);
                if ui.button("Create a node").clicked() {
                    let id =
                        self.state
                            .graph
                            .add_node("Node".to_owned(), DummyNodeData, |_g, _id| {});
                    self.state.node_order.push(id);
                    self.state
                        .node_positions
                        .insert(id, egui::Pos2 { x: 20.0, y: 20.0 });
                    DummyNodeTemplate.build_node(&mut self.state.graph, &mut self.user_state, id);
                    self.cached_text_graph_description = self.calculate_result();
                }
            });
        });
        egui::TopBottomPanel::top("result").show(ctx, |ui| {
            ui.label(&self.cached_text_graph_description);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let ret = self.state.draw_graph_editor(
                ui,
                AllMyNodeTemplates,
                &mut self.user_state,
                Vec::default(),
            );
            if !ret.node_responses.is_empty() {
                self.cached_text_graph_description = self.calculate_result();
            }
        });
    }
}

impl NodeGraphExampleSimple {
    fn calculate_result(&self) -> String {
        use std::collections::BTreeMap;
        struct NodeInfo {
            ins: Vec<NodeId>,
            leaf: bool,
            printed: std::cell::Cell<bool>,
        }
        let mut nodes: BTreeMap<NodeId, NodeInfo> = self
            .state
            .graph
            .iter_nodes()
            .map(|x| {
                (
                    x,
                    NodeInfo {
                        ins: vec![],
                        leaf: true,
                        printed: false.into(),
                    },
                )
            })
            .collect();
        for (input_id, output_id) in self.state.graph.iter_connections() {
            let input = self.state.graph.inputs.get(input_id).unwrap();
            let output = self.state.graph.outputs.get(output_id).unwrap();
            nodes.get_mut(&input.node).unwrap().ins.push(output.node);
            nodes.get_mut(&output.node).unwrap().leaf = false;
        }

        let mut out = String::with_capacity(128);

        fn printer(out: &mut String, nid: NodeId, db: &BTreeMap<NodeId, NodeInfo>) {
            let info = db.get(&nid).unwrap();
            if info.printed.get() {
                out.push_str("...");
                return;
            }
            info.printed.set(true);
            out.push_str("(node ");
            for input in &info.ins {
                printer(out, *input, db);
            }
            out.push_str(") ");
            info.printed.set(true);
        }

        for (id, info) in nodes.iter() {
            if !info.leaf {
                continue;
            }
            printer(&mut out, *id, &nodes);
        }

        out
    }
}
