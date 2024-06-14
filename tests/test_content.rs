use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn csv_has_column_names() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = assert_fs::NamedTempFile::new("sample.drawio")?;

    const DRAWIO_DATA: &str = r##"
<mxfile host="Electron" modified="2024-05-12T19:16:20.627Z" agent="Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) draw.io/24.2.5 Chrome/120.0.6099.109 Electron/28.1.0 Safari/537.36" etag="y88PgW05W1quMnjhG3vt" version="24.2.5" type="device">
    <diagram name="L1 diagram" id="lnKFR2A0_PlhCYDjk-47">
        <mxGraphModel dx="1147" dy="29" grid="1" gridSize="10" guides="0" tooltips="1" connect="0" arrows="0" fold="1" page="1" pageScale="1" pageWidth="1654" pageHeight="1169" math="0" shadow="0">
            <root>
                <mxCell id="0" />
            </root>
        </mxGraphModel>
    </diagram>
</mxfile>"##;

    input_file.write_str(DRAWIO_DATA)?;

    let mut cmd = Command::cargo_bin("tarpar")?;
    cmd.arg(input_file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(r##"sort;"object type";"object";"value";"action";"tags";"tooltip";"team";"tasks";"type";"color text";"color line";"layer";"diagram";"drawio";"id";"parent_id";"##));

    Ok(())
}

#[test]
/// String values should be cleaned from new lines, double quotes, semicolons
fn csv_has_normalized_string_values() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = assert_fs::NamedTempFile::new("sample.drawio")?;

    const DRAWIO_DATA: &str = r##"
<mxfile host="Electron" modified="2024-05-12T19:16:20.627Z" agent="Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) draw.io/24.2.5 Chrome/120.0.6099.109 Electron/28.1.0 Safari/537.36" etag="y88PgW05W1quMnjhG3vt" version="24.2.5" type="device">
    <diagram name="L1 diagram" id="lnKFR2A0_PlhCYDjk-47">
        <mxGraphModel dx="1147" dy="29" grid="1" gridSize="10" guides="0" tooltips="1" connect="0" arrows="0" fold="1" page="1" pageScale="1" pageWidth="1654" pageHeight="1169" math="0" shadow="0">
            <root>
                <mxCell id="0" />
                <mxCell id="w1Yb3G1GzCkss31SYDfo-59" value="Сетевой периметр Компании &quot;Стройка Века&quot;" style="rounded=0;whiteSpace=wrap;html=1;fillColor=default;verticalAlign=top;align=right;dashed=1;dashPattern=1 1;shadow=0;movable=1;resizable=1;rotatable=1;deletable=1;editable=1;locked=0;connectable=1;" parent="1" vertex="1">
                    <mxGeometry x="270" y="1410" width="1170" height="620" as="geometry" />
                </mxCell>
                <object label="Ромашка" placeholders="1" tooltip="" team="Финансы - Базовый учет" tasks="" tags="этап_1" id="w1Yb3G1GzCkss31SYDfo-33">
                    <mxCell style="swimlane;strokeColor=#007FFF;rounded=0;strokeWidth=1;fontStyle=1;childLayout=stackLayout;horizontal=1;startSize=26;fillColor=default;horizontalStack=0;resizeParent=1;resizeParentMax=0;resizeLast=0;collapsible=0;marginBottom=0;whiteSpace=wrap;labelBackgroundColor=none;swimlaneFillColor=#FFFFFF;swimlaneLine=1;shadow=1;glass=0;allowArrows=1;fixDash=0;expand=1;movableLabel=0;rotatable=0;noLabel=0;snapToPoint=0;perimeter=rectanglePerimeter;metaEdit=0;resizeWidth=0;resizeHeight=0;cloneable=1;deletable=1;treeFolding=0;treeMoving=0;pointerEvents=0;enumerate=0;comic=0;movable=1;resizable=1;editable=1;locked=0;connectable=1;autosize=0;spacing=0;" parent="1" vertex="1">
                        <mxGeometry x="510" y="1580" width="190" height="100" as="geometry">
                            <mxRectangle x="1360" y="1390" width="90" height="30" as="alternateBounds" />
                        </mxGeometry>
                    </mxCell>
                </object>
                <UserObject label="Дашборд финансовый; экономический" tooltip="" team="" tasks="" placeholders="1" id="w1Yb3G1GzCkss31SYDfo-34">
                    <mxCell style="text;strokeColor=none;fillColor=none;align=left;verticalAlign=middle;spacingLeft=0;spacingRight=0;overflow=hidden;rotatable=0;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;whiteSpace=wrap;fontColor=#000000;noLabel=0;expand=1;enumerate=0;textShadow=0;labelPosition=center;verticalLabelPosition=middle;spacing=3;" parent="w1Yb3G1GzCkss31SYDfo-33" vertex="1">
                        <mxGeometry y="26" width="190" height="34" as="geometry" />
                    </mxCell>
                </UserObject>
                <UserObject label="Расчет показателей&#xa;временный" tooltip="" team="" tasks="" placeholders="1" id="w1Yb3G1GzCkss31SYDfo-48">
                    <mxCell style="text;strokeColor=none;fillColor=none;align=left;verticalAlign=middle;spacingLeft=0;spacingRight=0;overflow=hidden;rotatable=0;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;whiteSpace=wrap;fontColor=#000000;noLabel=0;expand=1;enumerate=0;textShadow=0;labelPosition=center;verticalLabelPosition=middle;spacing=3;" parent="w1Yb3G1GzCkss31SYDfo-33" vertex="1">
                        <mxGeometry y="60" width="190" height="40" as="geometry" />
                    </mxCell>
                </UserObject>
            </root>
        </mxGraphModel>
    </diagram>
</mxfile>"##;

    input_file.write_str(DRAWIO_DATA)?;

    let mut cmd = Command::cargo_bin("tarpar")?;
    cmd.arg(input_file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            r##""Сетевой периметр Компании 'Стройка Века'""##, // replaced " with '
        ))
        .stdout(predicate::str::contains(
            r##""Дашборд финансовый, экономический""##, // replaced ; with ,
        ))
        .stdout(predicate::str::contains(
            r##""Расчет показателей временный""##, // replaced \n with space
        ));

    Ok(())
}
