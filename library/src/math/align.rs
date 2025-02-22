use super::*;

/// A math alignment point: `&`, `&&`.
///
/// Display: Alignment Point
/// Category: math
#[element(LayoutMath)]
pub struct AlignPointElem {}

impl LayoutMath for AlignPointElem {
    fn layout_math(&self, ctx: &mut MathContext) -> SourceResult<()> {
        ctx.push(MathFragment::Align);
        Ok(())
    }
}

/// Determine the position of the alignment points.
pub(super) fn alignments(rows: &[MathRow]) -> Vec<Abs> {
    let mut widths = Vec::<Abs>::new();

    for row in rows {
        let mut width = Abs::zero();
        let mut alignment_index = 0;
        for fragment in row.iter() {
            if matches!(fragment, MathFragment::Align) {
                if alignment_index < widths.len() {
                    widths[alignment_index].set_max(width);
                } else {
                    widths.push(width);
                }
                width = Abs::zero();
                alignment_index += 1;
            } else {
                width += fragment.width();
            }
        }
    }

    let mut points = widths;
    for i in 1..points.len() {
        let prev = points[i - 1];
        points[i] += prev;
    }
    points
}
