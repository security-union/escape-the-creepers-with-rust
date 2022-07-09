use yew::{prelude::*, virtual_dom::VNode};

#[derive(PartialEq, Properties)]
struct CellProps {
    row: i32,
    column: i32,
}
#[function_component(Cell)]
fn cell(p: &CellProps) -> Html {
    let CellProps { row, column } = p;
    html! {
        <div class = "cell">
            {row}{","}{column}
        </div>
    }
}

const ROWS: i32 = 24;
const COLUMNS: i32 = 12;

#[function_component(App)]
fn app_component() -> Html {
    fn column_generator(column: i32) -> Vec<VNode> {
        let rows: Vec<i32> = (0..ROWS).collect();
        rows.iter()
            .map(|row| {
                html! {
                    <Cell row={*row} column={column}/>
                }
            })
            .collect()
    }

    fn row_generator() -> Vec<VNode> {
        let rows: Vec<i32> = (0..COLUMNS).collect();
        rows.iter()
            .map(|j| {
                html! {
                    <>
                        {column_generator(*j)}
                    </>
                }
            })
            .collect()
    }
    html! {
        <div class="grid">
            {row_generator()}
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
