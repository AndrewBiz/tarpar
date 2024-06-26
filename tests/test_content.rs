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

mod sample {
    pub const DRAWIO_DATA: &str = r##"
<mxfile host="Electron" modified="2024-06-14T13:55:49.990Z" agent="Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) draw.io/24.5.3 Chrome/124.0.6367.207 Electron/30.0.6 Safari/537.36" etag="jYwSwvvK3cMMZgvTiuwF" version="24.5.3" type="device">
  <diagram name="L1 diagram" id="lnKFR2A0_PlhCYDjk-47">
    <mxGraphModel dx="1420" dy="-345" grid="1" gridSize="10" guides="0" tooltips="1" connect="0" arrows="0" fold="1" page="1" pageScale="1" pageWidth="1654" pageHeight="1169" math="0" shadow="0">
      <root>
        <mxCell id="0" />
        <mxCell id="1" value="пример схемы" style="" parent="0" />
        <mxCell id="w1Yb3G1GzCkss31SYDfo-59" value="Сетевой периметр Компании Стройка Века" style="rounded=0;whiteSpace=wrap;html=1;fillColor=default;verticalAlign=top;align=right;dashed=1;dashPattern=1 1;shadow=0;movable=1;resizable=1;rotatable=1;deletable=1;editable=1;locked=0;connectable=1;" parent="1" vertex="1">
          <mxGeometry x="460" y="1410" width="930" height="350" as="geometry" />
        </mxCell>
        <object label="Ромашка" placeholders="1" tooltip="" team="Команда Ромашка" tasks="" tags="этап_1" id="w1Yb3G1GzCkss31SYDfo-33">
          <mxCell style="swimlane;strokeColor=#007FFF;rounded=0;strokeWidth=1;fontStyle=1;childLayout=stackLayout;horizontal=1;startSize=26;fillColor=default;horizontalStack=0;resizeParent=1;resizeParentMax=0;resizeLast=0;collapsible=0;marginBottom=0;whiteSpace=wrap;labelBackgroundColor=none;swimlaneFillColor=#FFFFFF;swimlaneLine=1;shadow=1;glass=0;allowArrows=1;fixDash=0;expand=1;movableLabel=0;rotatable=0;noLabel=0;snapToPoint=0;perimeter=rectanglePerimeter;metaEdit=0;resizeWidth=0;resizeHeight=0;cloneable=1;deletable=1;treeFolding=0;treeMoving=0;pointerEvents=0;enumerate=0;comic=0;movable=1;resizable=1;editable=1;locked=0;connectable=1;autosize=0;spacing=0;" parent="1" vertex="1">
            <mxGeometry x="520" y="1550" width="190" height="100" as="geometry">
              <mxRectangle x="1360" y="1390" width="90" height="30" as="alternateBounds" />
            </mxGeometry>
          </mxCell>
        </object>
        <UserObject label="Дашборд финансовый, экономический" tooltip="" team="" tasks="" placeholders="1" id="w1Yb3G1GzCkss31SYDfo-34">
          <mxCell style="text;strokeColor=none;fillColor=none;align=left;verticalAlign=middle;spacingLeft=0;spacingRight=0;overflow=hidden;rotatable=0;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;whiteSpace=wrap;fontColor=#000000;noLabel=0;expand=1;enumerate=0;textShadow=0;labelPosition=center;verticalLabelPosition=middle;spacing=3;" parent="w1Yb3G1GzCkss31SYDfo-33" vertex="1">
            <mxGeometry y="26" width="190" height="34" as="geometry" />
          </mxCell>
        </UserObject>
        <UserObject label="Расчет показателей временный" tooltip="" team="" tasks="" placeholders="1" id="w1Yb3G1GzCkss31SYDfo-48">
          <mxCell style="text;strokeColor=none;fillColor=none;align=left;verticalAlign=middle;spacingLeft=0;spacingRight=0;overflow=hidden;rotatable=0;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;whiteSpace=wrap;fontColor=#007FFF;noLabel=0;expand=1;enumerate=0;textShadow=0;labelPosition=center;verticalLabelPosition=middle;spacing=3;fontFamily=Helvetica;fontSize=12;" parent="w1Yb3G1GzCkss31SYDfo-33" vertex="1">
            <mxGeometry y="60" width="190" height="40" as="geometry" />
          </mxCell>
        </UserObject>
        <object label="Лютик" placeholders="1" tooltip="" team="команда Лютик" tasks="" tags="этап_1" id="w1Yb3G1GzCkss31SYDfo-35">
          <mxCell style="swimlane;strokeColor=#007FFF;rounded=0;strokeWidth=1;fontStyle=1;childLayout=stackLayout;horizontal=1;startSize=26;fillColor=default;horizontalStack=0;resizeParent=1;resizeParentMax=0;resizeLast=0;collapsible=0;marginBottom=0;whiteSpace=wrap;labelBackgroundColor=none;swimlaneFillColor=#FFFFFF;swimlaneLine=1;shadow=1;glass=0;allowArrows=1;fixDash=0;expand=1;movableLabel=0;rotatable=0;noLabel=0;snapToPoint=0;perimeter=rectanglePerimeter;metaEdit=0;resizeWidth=0;resizeHeight=0;cloneable=1;deletable=1;treeFolding=0;treeMoving=0;pointerEvents=0;enumerate=0;comic=0;movable=1;resizable=1;editable=1;locked=0;connectable=1;autosize=0;spacing=0;" parent="1" vertex="1">
            <mxGeometry x="910" y="1560" width="200" height="122" as="geometry">
              <mxRectangle x="1360" y="1390" width="90" height="30" as="alternateBounds" />
            </mxGeometry>
          </mxCell>
        </object>
        <UserObject label="Учет первичных документов" tooltip="" team="" tasks="" placeholders="1" id="w1Yb3G1GzCkss31SYDfo-36">
          <mxCell style="text;strokeColor=none;fillColor=none;align=left;verticalAlign=middle;spacingLeft=0;spacingRight=0;overflow=hidden;rotatable=0;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;whiteSpace=wrap;fontColor=#000000;noLabel=0;expand=1;enumerate=0;textShadow=0;labelPosition=center;verticalLabelPosition=middle;spacing=3;" parent="w1Yb3G1GzCkss31SYDfo-35" vertex="1">
            <mxGeometry y="26" width="200" height="24" as="geometry" />
          </mxCell>
        </UserObject>
        <UserObject label="+ Первичный документ №2" tooltip="" team="команда Лютик-2" tasks="" placeholders="1" id="w1Yb3G1GzCkss31SYDfo-37">
          <mxCell style="text;strokeColor=none;fillColor=none;align=left;verticalAlign=middle;spacingLeft=0;spacingRight=0;overflow=hidden;rotatable=0;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;whiteSpace=wrap;fontColor=#009900;expand=1;enumerate=0;textShadow=0;labelPosition=center;verticalLabelPosition=middle;spacing=3;" parent="w1Yb3G1GzCkss31SYDfo-35" vertex="1">
            <mxGeometry y="50" width="200" height="24" as="geometry" />
          </mxCell>
        </UserObject>
        <UserObject label="* Первичный документ №3" tooltip="" team="" tasks="" placeholders="1" id="w1Yb3G1GzCkss31SYDfo-38">
          <mxCell style="text;strokeColor=none;fillColor=none;align=left;verticalAlign=middle;spacingLeft=0;spacingRight=0;overflow=hidden;rotatable=0;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;whiteSpace=wrap;fontColor=#007FFF;expand=1;enumerate=0;textShadow=0;labelPosition=center;verticalLabelPosition=middle;spacing=3;" parent="w1Yb3G1GzCkss31SYDfo-35" vertex="1">
            <mxGeometry y="74" width="200" height="24" as="geometry" />
          </mxCell>
        </UserObject>
        <UserObject label="- Отчет Баланс старый" tooltip="" team="" tasks="" placeholders="1" id="w1Yb3G1GzCkss31SYDfo-39">
          <mxCell style="text;strokeColor=none;fillColor=none;align=left;verticalAlign=middle;spacingLeft=0;spacingRight=0;overflow=hidden;rotatable=0;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;whiteSpace=wrap;fontColor=#FF0000;expand=1;enumerate=0;textShadow=0;labelPosition=center;verticalLabelPosition=middle;spacing=3;" parent="w1Yb3G1GzCkss31SYDfo-35" vertex="1">
            <mxGeometry y="98" width="200" height="24" as="geometry" />
          </mxCell>
        </UserObject>
        <object label="Бухгалтер" placeholders="1" id="w1Yb3G1GzCkss31SYDfo-45">
          <mxCell style="shape=umlActor;verticalLabelPosition=bottom;verticalAlign=top;outlineConnect=0;movableLabel=1;resizable=0;rotatable=0;treeFolding=0;treeMoving=0;enumerate=0;comic=0;resizeWidth=0;recursiveResize=0;textShadow=0;shadow=1;" parent="1" vertex="1">
            <mxGeometry x="1290" y="1550" width="30" height="60" as="geometry">
              <mxPoint x="-1" y="-8" as="offset" />
            </mxGeometry>
          </mxCell>
        </object>
        <UserObject label="учетные&#xa;действия" tooltip="" team="" tasks="" placeholders="1" id="w1Yb3G1GzCkss31SYDfo-46">
          <mxCell style="edgeStyle=orthogonalEdgeStyle;rounded=1;orthogonalLoop=1;jettySize=auto;fontColor=#000000;strokeColor=#000000;curved=0;fillOpacity=100;metaEdit=0;noJump=0;ignoreEdge=0;orthogonal=1;enumerate=0;bendable=1;perimeterSpacing=3;comic=0;arcSize=15;labelBackgroundColor=none;spacing=0;shadow=0;horizontal=1;startArrow=classic;startFill=1;jumpStyle=arc;" parent="1" source="w1Yb3G1GzCkss31SYDfo-45" target="w1Yb3G1GzCkss31SYDfo-35" edge="1">
            <mxGeometry x="0.0047" relative="1" as="geometry">
              <mxPoint x="1334" y="1830" as="sourcePoint" />
              <mxPoint x="1200" y="1830" as="targetPoint" />
              <mxPoint y="1" as="offset" />
            </mxGeometry>
          </mxCell>
        </UserObject>
        <UserObject label="ИВ 1" tooltip="" team="" tasks="" placeholders="1" tags="этап_1" id="w1Yb3G1GzCkss31SYDfo-56">
          <mxCell style="edgeStyle=orthogonalEdgeStyle;rounded=1;orthogonalLoop=1;jettySize=auto;fontColor=#000000;strokeColor=#007FFF;curved=0;fillOpacity=100;metaEdit=0;noJump=0;ignoreEdge=0;orthogonal=1;enumerate=0;bendable=1;perimeterSpacing=3;comic=0;arcSize=15;labelBackgroundColor=none;spacing=0;shadow=0;horizontal=1;jumpStyle=arc;" parent="1" source="w1Yb3G1GzCkss31SYDfo-38" target="w1Yb3G1GzCkss31SYDfo-48" edge="1">
            <mxGeometry x="0.2667" y="-10" relative="1" as="geometry">
              <mxPoint x="700" y="1500" as="sourcePoint" />
              <mxPoint x="802" y="1343" as="targetPoint" />
              <Array as="points">
                <mxPoint x="860" y="1646" />
                <mxPoint x="860" y="1630" />
              </Array>
              <mxPoint as="offset" />
            </mxGeometry>
          </mxCell>
        </UserObject>
        <mxCell id="MeAyv4VoMN-YbbC7TVDd-1" value="ИВ 2" style="edgeLabel;align=center;verticalAlign=middle;resizable=0;points=[];fontColor=#FF0000;" vertex="1" connectable="0" parent="w1Yb3G1GzCkss31SYDfo-56">
          <mxGeometry x="-0.1619" y="3" relative="1" as="geometry">
            <mxPoint x="-45" y="7" as="offset" />
          </mxGeometry>
        </mxCell>
        <mxCell id="MeAyv4VoMN-YbbC7TVDd-3" value="ИВ 3" style="edgeStyle=orthogonalEdgeStyle;curved=0;rounded=1;comic=0;jumpStyle=arc;orthogonalLoop=1;jettySize=auto;html=1;shadow=0;strokeColor=#000000;align=center;verticalAlign=middle;spacing=0;arcSize=15;fontFamily=Helvetica;fontSize=11;fontColor=#000000;labelBackgroundColor=none;endArrow=classic;" edge="1" parent="1" source="w1Yb3G1GzCkss31SYDfo-37" target="w1Yb3G1GzCkss31SYDfo-34">
          <mxGeometry x="0.214" y="-13" relative="1" as="geometry">
            <Array as="points">
              <mxPoint x="870" y="1622" />
              <mxPoint x="870" y="1593" />
            </Array>
            <mxPoint as="offset" />
          </mxGeometry>
        </mxCell>
        <object label="ИВ 4" team="Команда интеграции" id="MeAyv4VoMN-YbbC7TVDd-4">
          <mxCell style="edgeLabel;html=1;align=center;verticalAlign=middle;resizable=0;points=[];fontFamily=Helvetica;fontSize=11;fontColor=#000000;labelBackgroundColor=none;" vertex="1" connectable="0" parent="MeAyv4VoMN-YbbC7TVDd-3">
            <mxGeometry x="-0.1019" y="1" relative="1" as="geometry">
              <mxPoint x="-36" y="6" as="offset" />
            </mxGeometry>
          </mxCell>
        </object>
      </root>
    </mxGraphModel>
  </diagram>
</mxfile>
"##;
}

#[test]
fn team_field_for_functions_inherited_from_system() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = assert_fs::NamedTempFile::new("sample.drawio")?;

    input_file.write_str(sample::DRAWIO_DATA)?;

    let mut cmd = Command::cargo_bin("tarpar")?;
    cmd.arg(input_file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            r##""Дашборд финансовый, экономический";"";"";"";"Команда Ромашка";"";"SystemFunction""##,
        ))
        .stdout(predicate::str::contains(
            r##""Расчет показателей временный";"Доработать";"";"";"Команда Ромашка";"";"SystemFunction""##,
        ))
    ;

    Ok(())
}

#[test]
fn team_field_for_integrations_inherited_from_source_system_or_function(
) -> Result<(), Box<dyn std::error::Error>> {
    let input_file = assert_fs::NamedTempFile::new("sample.drawio")?;

    input_file.write_str(sample::DRAWIO_DATA)?;

    let mut cmd = Command::cargo_bin("tarpar")?;
    cmd.arg(input_file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            r##""Лютик <--> Ромашка";"ИВ 1";"Доработать";"этап_1";"";"команда Лютик";"";"Link";"##, // inherit team from source system
        ))
        .stdout(predicate::str::contains(
            r##""Лютик <--> Ромашка";"ИВ 2";"Вывести из эксплуатации";"";"";"команда Лютик";"";"LinkLabel";"##, // inherit team from source system
        ))
        .stdout(predicate::str::contains(
            r##""Лютик <--> Ромашка";"ИВ 3";"";"";"";"команда Лютик-2";"";"Link";"##, // inherit team from source system_function
        ))
        .stdout(predicate::str::contains(
            r##""Лютик <--> Ромашка";"ИВ 4";"";"";"";"Команда интеграции";"";"LinkLabel";"##, // don't inherit team here
        ));

    Ok(())
}
