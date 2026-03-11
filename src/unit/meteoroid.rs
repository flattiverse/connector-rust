use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, UnitBase, UnitExt, UnitExtSealed, UnitKind};
use crate::utils::Readable;
use num_enum::FromPrimitive;
use std::sync::Weak;

/// A meteoroid.
#[derive(Debug, Clone)]
pub struct Meteoroid {
    base: UnitBase,
    steady: SteadyUnit,
    r#type: MeteoroidType,
    metal: f32,
    carbon: f32,
    hydrogen: f32,
    silicon: f32,
}

impl Meteoroid {
    pub(crate) fn read(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            base: UnitBase::new(cluster, name),
            steady: SteadyUnit::read(reader),
            r#type: MeteoroidType::from_primitive(reader.read_byte()),
            metal: reader.read_f32(),
            carbon: reader.read_f32(),
            hydrogen: reader.read_f32(),
            silicon: reader.read_f32(),
        }
    }

    /// Visual type of the meteoroid.
    #[inline]
    pub fn r#type(&self) -> MeteoroidType {
        self.r#type
    }

    /// Metal richness of this meteoroid.
    #[inline]
    pub fn metal(&self) -> f32 {
        self.metal
    }

    /// Carbon richness of this meteoroid.
    #[inline]
    pub fn carbon(&self) -> f32 {
        self.carbon
    }

    /// Hydrogen richness of this meteoroid.
    #[inline]
    pub fn hydrogen(&self) -> f32 {
        self.hydrogen
    }

    /// Silicon richness of this meteoroid.
    #[inline]
    pub fn silicon(&self) -> f32 {
        self.silicon
    }
}

impl AsRef<UnitBase> for Meteoroid {
    #[inline]
    fn as_ref(&self) -> &UnitBase {
        &self.base
    }
}

impl AsRef<SteadyUnit> for Meteoroid {
    #[inline]
    fn as_ref(&self) -> &SteadyUnit {
        &self.steady
    }
}

impl<'a> UnitExtSealed<'a> for &'a Meteoroid {
    type Parent = (&'a UnitBase, &'a SteadyUnit);

    fn parent(self) -> Self::Parent {
        (&self.base, &self.steady)
    }
}

impl<'a> UnitExt<'a> for &'a Meteoroid {
    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::Meteoroid
    }
}

/// Describes the visual archetype of a meteoroid.
#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    num_enum::FromPrimitive,
    num_enum::IntoPrimitive,
    strum::EnumIter,
    strum::AsRefStr,
)]
pub enum MeteoroidType {
    /// # Summary
    ///
    /// A familiar gray rock with speckles and fracture lines, the kind of space debris that looks exactly like "a rock in space."
    /// It tumbles with uncomplicated honesty and rarely surprises anyone-useful for calibrating expectations and lowering blood pressure.
    /// The universe produces these in bulk, like it got bored and started copy-pasting.
    StonyFragment,

    /// # Summary
    ///
    /// A compact, dark-metal lump that catches highlights in sharp flashes, like a coin flipping forever in slow motion.
    /// It looks small until you notice how stubbornly it holds its presence, dragging a faint wake of smaller grit behind it.
    /// Old-timers swear these are the "bones" of larger bodies, condensed into dense little problems that refuse to break politely.
    ///
    /// # Remarks
    ///
    /// Expected resources: Metal.
    MetallicSlug,

    /// # Summary
    ///
    /// A soot-black chunk with a dull surface and crumbly edges, as if it is embarrassed to be seen under direct light.
    /// It sheds fine dust when warmed, leaving a smoky ribbon that makes it look dramatic in the worst possible way.
    /// Collectors like the romance of it-ancient, primitive, quietly strange-while practical crews just call it "the one that gets everywhere."
    ///
    /// # Remarks
    ///
    /// Expected resources: Carbon.
    CarbonaceousChunk,

    /// # Summary
    ///
    /// A pale, translucent fragment that wears a faint halo when it drifts through warmer regions, like a ghost trying to remember solidity.
    /// Its surface is pocked and layered, alternating milky bands and clear glassy patches that sparkle then vanish as it rotates.
    /// It feels alive in a way rocks should not: always changing, always faintly shedding, always reminding you that "solid" is sometimes a temporary agreement.
    ///
    /// # Remarks
    ///
    /// Expected resources: Hydrogen.
    CometaryIceFragment,

    /// # Summary
    ///
    /// A marbled shard of mixed textures-bright metallic veins threaded through darker stone-like someone poured molten metal into a cracked statue.
    /// It reflects light in inconsistent flashes, never quite symmetrical, always a little hostile to neat geometry.
    /// Every face looks like it belongs to a different parent body, and that patchwork quality gives it a reputation for being stubborn, unpredictable, and oddly charismatic.
    StonyIronShard,

    /// # Summary
    ///
    /// A glossy, obsidian-like body with smooth curves and mirror flashes that make it look briefly artificial at certain angles.
    /// It is the kind of object that triggers arguments: "Is that manufactured?" "No, it is just annoyingly pretty."
    /// Tiny bubbles and frozen ripples betray a violent birth, cooled too fast to be polite, leaving a sleek black relic that feels like a fragment of a broken window to nowhere.
    GlassyTektite,

    /// # Summary
    ///
    /// A scarred, dark shard whose surface looks sandblasted and heat-brushed, with shallow pits and sharp edges that catch harsh highlights.
    /// It carries the aesthetic of "been through something," even when no one agrees on what that something was.
    /// In motion it appears calmer than it is, turning slowly like a deliberate threat-an object that seems to have learned endurance the hard way, and kept the look as a souvenir.
    IrradiatedShard,

    /// # Summary
    ///
    /// A lump that refuses to look consistent: one side seems overly dark, the other oddly bright, as if the shadows cannot agree where to sit.
    /// Its silhouette reads different from different angles, and even careful observers find themselves second-guessing distance and size.
    /// It is the sort of thing crews nickname out of self-defense-The Joke, The Knot, The Bad Angle-because giving it a silly name feels safer than admitting it makes your eyes uncomfortable.
    AnomalousMass,

    /// # Summary
    ///
    /// Not a single body but a glittering haze of countless tiny flecks, like a strip of starlight someone spilled and forgot to clean up.
    /// From a distance it is beautiful-soft shimmer, gentle sparkle-up close it becomes an abrasive fog with the personality of a belt sander.
    /// The swarm's charm is purely aesthetic, which is fitting: in the Flattiverse, "beautiful but untouchable" is practically a design philosophy.
    MicroMeteorSwarm,

    /// # Summary
    ///
    /// A hollow-looking cinder with a pumice texture, full of tiny voids that make it seem lighter than it has any right to be.
    /// It catches light in a flattering way-soft highlights, gentle shading-like it was designed to look valuable.
    /// Up close it is mostly drama and very little substance, the kind of object that teaches crews to stop judging rocks by their sparkle.
    /// It is still popular, though, because everyone likes something that looks good in screenshots.
    HollowCinder,

    /// # Summary
    ///
    /// A ribbon of old slag and fused grit, stretched into a lopsided "comma" by ancient impacts and slow collisions.
    /// It has streaks and layers like geological sediment, except the colors do not quite make sense together.
    /// Some swear it is the remains of forgotten industry; others call it perfectly normal space trash with aspirations.
    /// Either way, it drifts like an accidental sculpture-proof that the belt produces art even when nobody asked for it.
    FusedSlagRibbon,

    /// # Summary
    ///
    /// A long, needle-like shard that looks less like a broken boulder and more like a snapped spear tip, thin enough to feel wrong.
    /// It rotates slowly, throwing tight flashes of light that make it seem to wink at passing ships.
    /// Nobody agrees on how such a shape survives intact for long, which is why it is always the subject of confident explanations and immediate skepticism.
    /// It is a belt curiosity: elegant, suspicious, and somehow always pointing at something important-until it is not.
    NeedleShard,

    /// # Summary
    ///
    /// A chalk-white flake with layered edges, like a piece of porcelain chipped off an antique plate and tossed into orbit.
    /// It looks delicate, even refined, and then you notice the jagged fractures and the faint dust cloud trailing behind it.
    /// It has the vibe of something that should be indoors, on a shelf, not tumbling through vacuum with a quiet sense of offended dignity.
    /// Crews like to photograph these because they look "too clean," which makes everyone else immediately distrust them.
    PorcelainFlake,

    #[num_enum(catch_all)]
    Unknown(u8),
}

impl MeteoroidType {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
