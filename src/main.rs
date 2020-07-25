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
            let child = node.nodes.iter().find(|n| want == n.id);
            match child {
                Some(c) => get_focused(c),
                None => None
            }
        } else {
            None
        }
    }
}

// returns what split is necessary
fn make_command(con : &mut I3Connection) -> String {
    let mut response = String::from("splith");
    let tree = con.get_tree().unwrap();
    let focused_node = get_focused(&tree);
    let node : &Node;

    match focused_node {
        Some(n) => {node = n},
        None => return response,
    };

    if node.layout != NodeLayout::Stacked && node.layout != NodeLayout::Tabbed && node.nodetype != NodeType::FloatingCon {
        let width = node.rect.2;
        let height = node.rect.3;
        if height > width {
            response = String::from("splitv");
        }
    }

    return response;
}

fn window_event_handle(e : WindowEventInfo) {
    if e.change == WindowChange::Focus {
        let mut con = I3Connection::connect().unwrap();
        let command = make_command(&mut con);
        con.run_command(command.as_str()).unwrap();
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
