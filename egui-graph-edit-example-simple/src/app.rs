use std::borrow::Cow;

use eframe::egui;
use egui_graph_edit::*;

/// Additional (besides inputs and outputs) state to be stored inside each node.
#[derive(Debug)]
pub struct DummyNodeData;

// Connection variant. Equal DataType means input port is compatible with output port.
// Typically an enum, but this example has only one connection type (any output can be connected to any input),
// so this type is dummied out.
#[derive(PartialEq, Eq, Debug)]
pub struct DummyDataType;

/// Type of the editable value that is used as a fallback for unconnected input node,
/// i.e. when some input to a node can be either constant or taken from another node,
/// this defines how to store that constant.
///
/// This example does not feature editable content within nodes, so this type is dummy.
#[derive(Copy, Clone, Debug, Default)]
pub struct DummyValueType;

/// Typically an enum that lists node types.
/// In this example there is only one node type ("Node"),
/// so no this type is dummy.
#[derive(Clone, Copy)]
pub struct DummyNodeTemplate;

/// Additional events that bubble up from `NodeDataTrait::bottom_ui` back to your app.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DummyResponse;

/// Supplementary, user-defined whole-graph state (i.e. a designated node)
/// Per-node supplemental infromation is stored elsewhere, not here.
///
/// This example does not use such state, so this type is dummied out.
type DummyGraphState = ();

/// Defines how to render edges (connections) between nodes
impl DataTypeTrait<DummyGraphState> for DummyDataType {
    fn data_type_color(&self, _user_state: &mut DummyGraphState) -> egui::Color32 {
        egui::Color32::from_rgb(238, 207, 60)
    }

    fn name(&self) -> Cow<'_, str> {
        "edge".into()
    }
}

/// Defines how to name and construct each node variant and what inputs and
/// outputs each node variant has.
impl NodeTemplateTrait for DummyNodeTemplate {
    type NodeData = DummyNodeData;
    type DataType = DummyDataType;
    type ValueType = DummyValueType;
    type UserState = DummyGraphState;
    type CategoryType = &'static str;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        "Node".into()
    }

    fn node_graph_label(&self, _user_state: &mut Self::UserState) -> String {
        "Node".to_owned()
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

/// Enumeration of all node variants to populate the context menu that allows creating nodes
pub struct AllMyNodeTemplates;
impl NodeTemplateIter for AllMyNodeTemplates {
    type Item = DummyNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        vec![DummyNodeTemplate]
    }
}

/// Defines how to render input's GUI when it is not connected.
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

/// Defines how to render node window (besides inputs and output ports)
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

/// Main graph editor type
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
    /// Text to display above the graph
    cached_text_graph_description: String,
}

impl eframe::App for NodeGraphExampleSimple {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Add a panel with buttons
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_switch(ui);
                if ui.button("Create a node").clicked() {
                    // Add a node to the database inside egui-graph-edit.
                    let id =
                        self.state
                            .graph
                            .add_node("Node".to_owned(), DummyNodeData, |_g, _id| {});
                    // Supplement z-order for the node (panic if missing)
                    self.state.node_order.push(id);
                    // Position the node within editor area (panic if missing)
                    self.state
                        .node_positions
                        .insert(id, egui::Pos2 { x: 20.0, y: 20.0 });
                    // Orientation of the node (panic if missing)
                    self.state
                        .node_orientations
                        .insert(id, NodeOrientation::LeftToRight);
                    // Fill in GUI within the node window, create inputs and outputs
                    DummyNodeTemplate.build_node(&mut self.state.graph, &mut self.user_state, id);
                    // Recalculate the line to display above the graph
                    self.cached_text_graph_description = self.calculate_result();
                }
            });
        });
        // Add a panel where textual representation of the graph will be displayed
        egui::TopBottomPanel::top("result").show(ctx, |ui| {
            ui.label(&self.cached_text_graph_description);
        });

        // Add main panel with the interactive graph
        egui::CentralPanel::default().show(ctx, |ui| {
            // Triger graph display and obtain user interaction events, if any.
            let ret = self.state.draw_graph_editor(
                ui,
                AllMyNodeTemplates,
                &mut self.user_state,
                Vec::default(),
            );
            // On any significant user interaction (i.e. creating or removing a node,
            // adding or removing an edge) recalculate the line.
            //
            // Insignificant interactions (like dragging things around)
            // are not included in `node_responses`
            if !ret.node_responses.is_empty() {
                self.cached_text_graph_description = self.calculate_result();
            }
        });
    }
}

impl NodeGraphExampleSimple {
    /// Walk the graph and calculate an S-expression that is shaped like the graph
    fn calculate_result(&self) -> String {
        // BTreeMap instead of HashMap to avoid chaotic reorderings within the resulting text
        use std::collections::BTreeMap;
        // Information about node position within hierarchy.
        struct NodeInfo {
            ins: Vec<NodeId>,
            leaf: bool,
            /// Replace this node with "..." when printing,
            /// as it was already printed before; prevent endless recursion.
            printed: std::cell::Cell<bool>,
        }
        // Initial filling of the node table
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
        // Adding inner (i.e. parent) node pointers
        for (input_id, output_id) in self.state.graph.iter_connections() {
            // lookup information about connection details
            // (in our case, from where to where it is going)
            let input = self.state.graph.inputs.get(input_id).unwrap();
            let output = self.state.graph.outputs.get(output_id).unwrap();
            nodes.get_mut(&input.node).unwrap().ins.push(output.node);
            nodes.get_mut(&output.node).unwrap().leaf = false;
        }

        // Output buffer
        let mut out = String::with_capacity(128);

        // Recursive function to output the S-expression
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

        // Iterate leaf nodes and print them
        for (id, info) in nodes.iter() {
            if !info.leaf {
                continue;
            }
            printer(&mut out, *id, &nodes);
        }

        out
    }
}
