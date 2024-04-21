// use regex::Regex;

fn main() {
    println!("Usage:\n\tcargo run --example html-styles");

    let style = r#"text;strokeColor=none;fillColor=none;align=left;verticalAlign=middle;spacingLeft=4;spacingRight=4;overflow=hidden;rotatable=0;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;whiteSpace=wrap;html=1;fontColor=#000000;"#;
    println!("{:#?}", style);
    println!("type = text: {}", style.contains("text;"));
    // stroke color - no matter
    // text color = in html label: font color="#ff0000"
    //              in html label: color: rgb(0, 153, 0);
    //              then in style: fontColor=#000000;

    let style = r#"shape=mxgraph.signs.safety.no_entry;html=1;pointerEvents=1;fillColor=#FF9933;strokeColor=#b85450;verticalLabelPosition=bottom;verticalAlign=top;align=center;sketch=0;aspect=fixed;"#;
    println!("{:#?}", style);
    println!("type = shape: {}", style.contains("shape="));
    // color - no matter

    // style='shape=umlActor;verticalLabelPosition=bottom;verticalAlign=top;html=1;outlineConnect=0;'
    // color - no matter

    // style='ellipse;shape=cloud;whiteSpace=wrap;html=1;strokeWidth=1;strokeColor=#007FFF;fontSize=14;fontColor=#4D4D4D;'
    // color - no matter

    let style="edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;entryX=0;entryY=0.5;entryDx=0;entryDy=0;";
    println!("{:#?}", style);
    println!("type = link: {}", style.contains("edgeStyle="));
    // color - strokeColor=#FF0000
    // label color - no matter

    let style="endArrow=classic;html=1;rounded=0;strokeColor=#FF0000;strokeWidth=2;fontColor=#000000;edgeStyle=orthogonalEdgeStyle;labelBackgroundColor=#FFFFFF;sketch=0;exitX=1;exitY=0.5;exitDx=0;exitDy=0;entryX=0.316;entryY=0.939;entryDx=0;entryDy=0;entryPerimeter=0;startArrow=classic;startFill=1;";
    println!("{:#?}", style);
    println!("type = link: {}", style.contains("edgeStyle="));
    // color - strokeColor=#FF0000
    // label color - no matter

    let style = "edgeLabel;html=1;align=center;verticalAlign=middle;resizable=0;points=[];";
    println!("{:#?}", style);
    println!("type = link_label: {}", style.contains("edgeLabel;"));
    // color - no matter

    let style = "group;strokeColor=none;strokeWidth=2;fontSize=14;fontColor=#4D4D4D;";
    println!("{:#?}", style);
    println!("type = group: {}", style.contains("group;"));
    // color - no matter

    let style="swimlane;strokeColor=#007FFF;rounded=0;strokeWidth=2;fontStyle=0;childLayout=stackLayout;horizontal=1;startSize=29;fillColor=#CCCCCC;horizontalStack=0;resizeParent=1;resizeParentMax=0;resizeLast=0;collapsible=1;marginBottom=0;whiteSpace=wrap;html=1;labelBackgroundColor=none;swimlaneFillColor=default;spacing=2;fontSize=14;fontColor=#4D4D4D;";
    println!("{:#?}", style);
    println!("type = system: {}", style.contains("swimlane;"));
    // color - strokeColor=#007FFF;
    // label color - no matter
}
