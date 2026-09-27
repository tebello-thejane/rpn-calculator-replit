//! Visual features module containing all optional display enhancements

#[cfg(feature = "stack-viz")]
pub mod stack_visualization;

#[cfg(feature = "latex-rendering")]
pub mod latex_rendering;

use crate::Calculator;

pub struct FeatureManager {
    #[cfg(feature = "stack-viz")]
    pub stack_viz: stack_visualization::StackVisualizer,
    
    #[cfg(feature = "latex-rendering")]
    pub latex_renderer: latex_rendering::LatexRenderer,
}

impl FeatureManager {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "stack-viz")]
            stack_viz: stack_visualization::StackVisualizer::new(),
            
            #[cfg(feature = "latex-rendering")]
            latex_renderer: latex_rendering::LatexRenderer::new(),
        }
    }

    pub fn display_stack(&self, calculator: &Calculator) -> String {
        #[cfg(feature = "stack-viz")]
        {
            self.stack_viz.render_stack(calculator.get_stack())
        }
        #[cfg(not(feature = "stack-viz"))]
        {
            calculator.display_stack()
        }
    }

    pub fn render_latex(&self, expression: &str) -> String {
        #[cfg(feature = "latex-rendering")]
        {
            self.latex_renderer.convert_rpn_to_latex(expression)
        }
        #[cfg(not(feature = "latex-rendering"))]
        {
            expression.to_string()
        }
    }
}

impl Default for FeatureManager {
    fn default() -> Self {
        Self::new()
    }
}