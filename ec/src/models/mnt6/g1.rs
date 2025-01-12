use crate::{
    mnt6::MNT6Parameters,
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
    AffineCurve,
};
use ark_ff::Fp3;

pub type G1Affine<P> = GroupAffine<<P as MNT6Parameters>::G1Parameters>;
pub type G1Projective<P> = GroupProjective<<P as MNT6Parameters>::G1Parameters>;

#[derive(Derivative)]
#[derivative(
    Copy(bound = "P: MNT6Parameters"),
    Clone(bound = "P: MNT6Parameters"),
    Debug(bound = "P: MNT6Parameters"),
    PartialEq(bound = "P: MNT6Parameters"),
    Eq(bound = "P: MNT6Parameters")
)]
pub struct G1Prepared<P: MNT6Parameters> {
    pub x: P::Fp,
    pub y: P::Fp,
    pub x_twist: Fp3<P::Fp3Config>,
    pub y_twist: Fp3<P::Fp3Config>,
}

impl<P: MNT6Parameters> From<G1Affine<P>> for G1Prepared<P> {
    fn from(g1: G1Affine<P>) -> Self {
        let mut x_twist = P::TWIST;
        x_twist.mul_assign_by_fp(&g1.x);

        let mut y_twist = P::TWIST;
        y_twist.mul_assign_by_fp(&g1.y);

        Self {
            x: g1.x,
            y: g1.y,
            x_twist,
            y_twist,
        }
    }
}

impl<P: MNT6Parameters> Default for G1Prepared<P> {
    fn default() -> Self {
        Self::from(G1Affine::<P>::prime_subgroup_generator())
    }
}
