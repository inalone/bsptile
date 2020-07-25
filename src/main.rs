extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::reply;
use i3ipc::reply::NodeLayout;
use i3ipc::reply::NodeType;
use i3ipc::reply::Node;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::Event;
use i3ipc::event::WindowEventInfo;
use i3ipc::event::inner::WindowChange;

// gets the currently focused window (implemented from https://github.com/tmerr/i3ipc-rs/issues/29)
fn get_focused(node : &reply::Node) -> Option<&Node> {
    if node.focused {
        Some(node)
    } else {
        if let Some(&want) = node.focus.get(0) {
            let child = node.nodes.iter().find(|n| want == n.id).unwrap();
            get_focused(child)
        } else {
            None
        }
    }
}

// returns what split is necessary
fn make_command(con : &mut I3Connection) -> &'static str {
    let tree = con.get_tree().unwrap();
    let focused_node = get_focused(&tree).unwrap();

    if focused_node.layout != NodeLayout::Stacked && focused_node.layout != NodeLayout::Tabbed && focused_node.nodetype != NodeType::FloatingCon {
        let width = focused_node.rect.2;
        let height = focused_node.rect.3;
        if height > width {
            return "splitv";
        }
    }

    return "splith";
}

fn window_event_handle(e : WindowEventInfo) {
    if e.change == WindowChange::Focus {
        let mut con = I3Connection::connect().unwrap();
        let command = make_command(&mut con);
        con.run_command(command).unwrap();
    }
}

fn main() {
    let mut listener = I3EventListener::connect().unwrap();
    listener.subscribe(&[Subscription::Window]).unwrap();

    for event in listener.listen() {
        match event.unwrap() {
            Event::WindowEvent(e) => window_event_handle(e),
            _ => unreachable!()
        }
    }
}
