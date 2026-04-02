use yew::prelude::*;
//use log::info;
//use common::*;
use common::haiku::HaikuLine;

#[derive(Properties, PartialEq)]
pub struct Props {
    //pub name: AttrValue,
    pub lines: Vec<HaikuLine>,
}


#[function_component(HaikuDraw)]
pub fn draw_haiku( Props { lines }: &Props ) -> HtmlResult {
    //info!("SLOW {:?}", selected_rotor_id);
    let lines_length: usize = lines.len();
    let gap: usize = 20;
    let max: usize = 220;
    let charwidth: usize = 24;
    let offset: usize = max / lines_length + charwidth - gap;
    // 3 * 24 / (4)
    let mut array: [usize; 3] = [0; 3];

    for n in 0..lines_length {
        array[n] = (offset+1)*n + charwidth;
    }

    Ok(
        html!{
            <svg viewBox="0 0 220 300" xmlns="http://www.w3.org/2000/svg">
            <style>
            {r#"text {
                      font-family: "Noto Serif JP", serif;
                      font-size: 22px;
                      //fill: none;
                      stroke: #efefef;
                      stroke-width: 1.3;
                      stroke-dasharray: 300;
                      stroke-dashoffset: 300;
                      animation: draw 2s ease forwards;
                    }
                    .line2 {
                      animation-delay: 1.8s;
                    }

                    .line3 {
                      animation-delay: 3.6s;
                    }

                    @keyframes draw {
                      to {
                        stroke-dashoffset: 0;
                      }"#}
            </style>
            <rect>
              <animate attributeName="x" from="0" to="100" dur="2s" begin="click" />
            </rect>

            for (idx, hl) in lines.iter().enumerate() {
                <text x={array[idx].to_string()} y=100 class={ format!("line{}", idx) }
                    text-anchor="middle" writing-mode="vertical-rl">
                   { hl.line.clone()  }
                    </text>
            }
            </svg>

        }
    )
}
