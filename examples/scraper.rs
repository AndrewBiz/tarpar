use scraper::{Html, Selector};
fn main() {
    println!("Usage:\n\tcargo run --example scraper");

    // let html = r#"
    // <ul>
    //     <li>Foo</li>
    //     <li>Bar</li>
    //     <li>Baz</li>
    // </ul>
    // "#;
    // let fragment = Html::parse_fragment(html);
    // let ul_selector = Selector::parse("ul").unwrap();
    // let li_selector = Selector::parse("li").unwrap();

    // let ul = fragment.select(&ul_selector).next().unwrap();
    // for element in ul.select(&li_selector) {
    //     println!("name = {}", element.value().name());
    // }

    // let fragment = Html::parse_fragment(r#"<font color="007fff">* Функция изменяется</font>"#);
    // Система приемник<br style="font-size: 8px;">полезных<br style="font-size: 8px;">данных
    // <font style="font-size: 11px;">S.CAT-S.BUD:&nbsp;<br style="border-color: var(--border-color);">РСС, КС-2</font>

    // let fragment = Html::parse_fragment(
    //     // ok
    //     r#"<font color="009900">-&nbsp;<span style="caret-color: rgb(0, 153, 0);">Инициир. партнерских соглашений?</span></font>"#,
    // );

    // let fragment = Html::parse_fragment(
    //     r#"Система приемник<br style="font-size: 8px;">полезных<br style="font-size: 8px;">данных"#,
    //     // tree: Tree { Fragment => { Element(<html>) => {
    //     //                                                Text("Система приемник"),
    //     //                                                Element(<br style="font-size: 8px;">),
    //     //                                                Text("полезных"),
    //     //                                                Element(<br style="font-size: 8px;">),
    //     //                                                Text("данных") } }
    // );

    // let fragment = Html::parse_fragment(
    //     r#"<font style="font-size: 11px;">S.CAT-S.BUD:&nbsp;<br style="border-color: var(--border-color);">РСС, КС-2</font>"#,
    // );

    // let fragment = Html::parse_fragment(r#"Прямоугольник с закругленными краями"#);

    // let fragment = Html::parse_fragment(
    //     r#"<h1>Схема листа 1</h1><p>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>"#,
    // );

    // let fragment =
    // Html::parse_fragment(r#"<font color="\#ff0000"><strike>Функция удаляется</strike></font>"#);

    // let fragment = Html::parse_fragment(r#"<font color="\#0000ff">- Бюджет G&amp;A</font>"#);
    let fragment = Html::parse_fragment(
        r#"<b>+ Договорной учет (договоры / ДС СМР / АХР / Коммерческие):<br></b><span style="white-space: pre;">  </span>+ Формирование карточки договора через интеграцию<br><span style="white-space: pre;">    </span>+ Контроль лимитов по договору<br><span style="white-space: pre;">       </span>+ Учет графика поставок/начислений<br><span style="white-space: pre;">   </span>+ Учет исполнения договора закрывающими и платежными документами<br><span style="white-space: pre;">     </span>+ Контроль сроков договоров<br><span style="white-space: pre;">  </span>+ Отображение план/факта ДДС<br><span style="white-space: pre;"> </span>+ Учет графика платежей<br><span style="white-space: pre;">      </span>+ Отправка уведомлений<br><span style="white-space: pre;">       </span>+ Поиск и фильтрация договоров<br><span style="white-space: pre;">      </span>+ Отчетность по договорам<b><br></b>"#,
    );

    println!("{:#?}", fragment);

    // let selector = Selector::parse(r#"font"#).unwrap();
    let selector = Selector::parse(r#"html"#).unwrap();

    let node = fragment.select(&selector).next().unwrap();
    println!("color = {:?}", node.value().attr("color"));

    let text = node.text().collect::<Vec<_>>();
    print!("TEXT = '");
    for t in text.iter() {
        print!("{} ", t);
    }
    println!("'");
}
