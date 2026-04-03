use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::steady::SteadyUnitInternal;
use crate::unit::unit::{Unit, UnitInternal};
use crate::unit::{AbstractSteadyUnit, SteadyUnit, UnitHierarchy, UnitKind};
use crate::utils::Atomic;
use crate::GameError;
use num_enum::FromPrimitive;
use std::sync::{Arc, Weak};

/// Moon map unit that can act as a mining target.
#[derive(Debug, Clone)]
pub struct Moon {
    parent: AbstractSteadyUnit,
    r#type: MoonType,
    metal: Atomic<f32>,
    carbon: Atomic<f32>,
    hydrogen: Atomic<f32>,
    silicon: Atomic<f32>,
}

impl Moon {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractSteadyUnit::new(cluster, name, reader)?,
            r#type: MoonType::from_primitive(reader.read_byte()),
            metal: Atomic::default(),
            carbon: Atomic::default(),
            hydrogen: Atomic::default(),
            silicon: Atomic::default(),
        }))
    }

    /// Visual type of the moon.
    #[inline]
    pub fn r#type(&self) -> MoonType {
        self.r#type
    }

    /// Metal richness of this moon for the current mining model.
    #[inline]
    pub fn metal(&self) -> f32 {
        self.metal.load()
    }

    /// Carbon richness of this moon for the current mining model.
    #[inline]
    pub fn carbon(&self) -> f32 {
        self.carbon.load()
    }

    /// Hydrogen richness of this moon for the current mining model.
    #[inline]
    pub fn hydrogen(&self) -> f32 {
        self.hydrogen.load()
    }

    /// Silicon richness of this moon for the current mining model.
    #[inline]
    pub fn silicon(&self) -> f32 {
        self.silicon.load()
    }
}

impl UnitInternal for Moon {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.metal.read(reader);
        self.carbon.read(reader);
        self.hydrogen.read(reader);
        self.silicon.read(reader);
    }
}

impl UnitHierarchy for Moon {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_moon(&self) -> Option<&Moon> {
        Some(self)
    }
}

impl Unit for Moon {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Moon
    }
}

impl SteadyUnitInternal for Moon {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for Moon {}

/// Describes the visual archetype of a moon.
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
pub enum MoonType {
    /// # Summary
    ///
    /// A cratered rock moon with crisp shadows and a surface that looks like ground-down stone and broken pottery.
    /// Its face is a museum of impacts: overlapping circles, raised rims, and long ejecta streaks that resemble brush marks.
    /// It is the dependable backdrop to grander planets-quiet, patient, and photogenic in a severe way-like a monochrome companion that makes everything else look more colorful by contrast.
    RockyMoon,

    /// # Summary
    ///
    /// A bright ice moon with subtle blue shading and spiderweb fractures that catch light like hairline cracks in glass.
    /// Near the limb, faint plumes may rise and fade, giving the impression of the moon breathing in slow motion.
    /// It looks clean enough to drink, which is of course hilarious, because you cannot land, cannot scoop, cannot even pretend-just orbit and stare at the universe's most expensive ice cube.
    ///
    /// # Remarks
    ///
    /// Expected resources: Hydrogen.
    IceMoon,

    /// # Summary
    ///
    /// A moon with a thin, stressed shell patterned by long dark lines and bright seams, like a frozen lake scaled up to planetary proportions.
    /// The cracks form networks that suggest tides working patiently from below, carving a secret geography you will never walk.
    /// When the light is right, the surface looks almost translucent at the edges, hinting at depth and motion underneath-an ocean story told entirely in ice-calligraphy.
    OceanMoon,

    /// # Summary
    ///
    /// A volcanic moon with glowing hotspots and dark ash plains that make it look like ember-studded charcoal.
    /// Its bright regions drift and reshape over time, while the darker fields spread in soft fans, as if someone is constantly repainting it with soot.
    /// It is uncomfortably pretty: the kind of light you want to watch, the kind of heat you do not want to meet.
    /// From orbit it reads like a warning label written in orange.
    VolcanicMoon,

    /// # Summary
    ///
    /// A captured asteroid moon-potato-shaped, lopsided, and unapologetically irregular-like the universe skipped quality control.
    /// Its surface is a jumble of ridges, shallow basins, and angular boulders that cast dramatic, jagged shadows.
    /// It has the charm of a junkyard mascot: not elegant, not smooth, but strangely lovable once you accept it will never look "planetary."
    /// If moons had personalities, this one would be the scrappy sidekick.
    ///
    /// # Remarks
    ///
    /// Expected resources: Metal.
    CapturedAsteroidMoon,

    /// # Summary
    ///
    /// A small shepherd moon skimming the edge of ring material, its surface dusted and polished by endless microscopic impacts.
    /// It appears brighter on some faces than others, as if constantly sanded by invisible hands.
    /// In wide shots it feels like a careful caretaker, keeping ring arcs tidy-then you zoom in and realize it is more like a pebble surviving in a hailstorm.
    /// It wears its job in scuffed highlights and softened contours.
    RingShepherdMoon,

    /// # Summary
    ///
    /// A "garden" moon seen as soft greens and browns under thin cloud veils, with gentle gradients rather than hard edges.
    /// It looks calm, almost domestic, like a world that decided to be pleasant on purpose.
    /// That calm is what makes it funny: crews spend hours admiring what seems like breathable beauty, then remember the Flattiverse rulebook-no landings, no walks, no fresh air, only wistful commentary over comms and a lot of screenshots.
    ///
    /// # Remarks
    ///
    /// Expected resources: Carbon.
    GardenMoon,

    /// # Summary
    ///
    /// A ruined moon etched with straight lines and odd symmetry, where surface patterns suggest old infrastructure half-buried under dust.
    /// Some regions look scraped clean, others mottled as if plated, leaving a patchwork of artificial geometry and natural erosion.
    /// It has the unsettling vibe of an abandoned workshop: tools put down mid-task, lights long gone, order still visible in the mess.
    /// The moon does not look dead so much as paused, waiting for someone to press play.
    RuinedMoon,

    /// # Summary
    ///
    /// A cryovolcanic moon where pale streaks radiate from vent regions like splashed paint, fresh lines laid over older, faded ones.
    /// The surface alternates between smooth, bright sheets and darker, fractured terrain, giving it a layered, time-lapse feel.
    /// From orbit it looks like a living sketchbook: repeated bursts, repeated patterns, repeated edits.
    /// It is the cold cousin of volcanic worlds-same drama, different palette-proof that "eruption" is a concept, not a temperature.
    CryovolcanicMoon,

    /// # Summary
    ///
    /// A shadow moon dominated by regions that never see direct sunlight, creating stark boundaries between bright rims and ink-dark interiors.
    /// The darkness is not empty; it has texture, subtle gradients, and occasional faint glints that feel almost like eyeshine.
    /// It is beautiful in the way deep caves are beautiful-mysterious, indifferent, and impossible to enter in this era.
    /// Every pass invites the same joke: "Let's go down there." Every pass ends the same way: "Right. Orbit-only. Again."
    ShadowMoon,

    /// # Summary
    ///
    /// A pale dustball moon with a surface so uniformly beige that it looks like it was rendered with a single color swatch.
    /// Subtle ripples and shallow hollows add just enough texture to keep it from being boring, which is impressive given how hard it tries.
    /// It is the moon you show new crew as a prank: "Look closely, it is spectacular." They do, they squint, they wait... and eventually realize the joke is that it is relentlessly average.
    /// Still, it photographs well beside dramatic planets-pure visual contrast, pure nothingness.
    BlandDustMoon,

    /// # Summary
    ///
    /// A glittering "mirror-dust" moon where fine reflective grains coat everything, making crater rims flash like sequins.
    /// It looks festive, almost cheerful, as if the universe accidentally invented holiday decoration in celestial form.
    /// Then the angle shifts and it turns stark, cold, and metallic in tone-beauty by glare, not by warmth.
    /// People always say they want to step onto that shimmering surface, and someone always replies: "Sure. In your next life. Orbit-only remembers."
    MirrorDustMoon,

    /// # Summary
    ///
    /// A smooth, glassy moon with broad, shallow basins and a surface sheen that makes it look freshly varnished.
    /// Highlights slide across it like liquid, and the crater rims appear softer than they should, as if time decided to sandpaper everything evenly.
    /// It is oddly serene-no dramatic scars, no loud textures-just an immaculate orb that feels like it belongs in a gallery.
    /// Crews love it because it makes every other moon look rugged; crews hate it because it makes them feel underdressed.
    GlassMarbleMoon,

    /// # Summary
    ///
    /// A twin-lobed moon shaped like two stones pressed together, casting a silhouette that always looks slightly ridiculous.
    /// Its surface is a patchwork of ridges and shallow hollows, with a seam-like boundary that catches the light in a neat curve.
    /// Everyone makes the same joke about it being "two moons that could not commit," and then immediately saves screenshots because it is genuinely charming.
    /// It is a reminder that gravity has a sense of humor, and the Flattiverse politely prevents you from ever poking the punchline.
    TwinLobeMoon,

    #[num_enum(catch_all)]
    Unknown(u8),
}

impl MoonType {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
