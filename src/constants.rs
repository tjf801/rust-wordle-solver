pub const WORDLE_NUM_GUESSES: usize = 6; // for testing porpoises 🐬

// #[rustc_layout_scalar_valid_range_start(b'A')]
// #[rustc_layout_scalar_valid_range_end(b'Z')]
// #[rustc_nonnull_optimization_guaranteed]
pub type WordleWord = [u8; 5];
pub type WordleAnswer = [u8; 5];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WordleWordScore {
	pub max_worst_case: u16,
	pub clue_dist_sum: u16
}

impl WordleWordScore {
	pub const MIN: Self = Self {max_worst_case: u16::MAX, clue_dist_sum: 0};
	pub const MAX: Self = Self {max_worst_case: u16::MIN, clue_dist_sum: u16::MAX};
}

impl Default for WordleWordScore {
	fn default() -> Self {
		Self::MIN
	}
}

impl PartialOrd for WordleWordScore {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for WordleWordScore {
	// isomorphic to the surreal numbers! nice
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match self.max_worst_case.cmp(&other.max_worst_case) {
			std::cmp::Ordering::Equal => self.clue_dist_sum.cmp(&other.clue_dist_sum),
			x => x
		}
	}
}


pub const NUM_WORDLE_CLUES: usize = 243;


#[rustc_layout_scalar_valid_range_end(242)]
#[rustc_nonnull_optimization_guaranteed]
#[derive(Clone, Copy, Debug, PartialEq, Eq)] // TODO: why is Eq not const :/
pub struct WordleClue(u8);

impl WordleClue {
	pub const fn new(value: u8) -> Self {
		if 243 <= value {panic!("WordleClue::new value is too large")}
		unsafe { WordleClue(value) }
	}
	pub const fn as_usize(self) -> usize {
		self.0 as usize
	}
}

impl Default for WordleClue {
	fn default() -> Self {
		WordleClue::new(0)
	}
}



pub const NUM_WORDLE_ANSWERS: usize = 2309;

// this is kinda gross lol
pub const WORDLE_ANSWERS: [WordleAnswer; NUM_WORDLE_ANSWERS] = [
	*b"ABACK", *b"ABASE", *b"ABATE", *b"ABBEY", *b"ABBOT", *b"ABHOR", *b"ABIDE", *b"ABLED", 
	*b"ABODE", *b"ABORT", *b"ABOUT", *b"ABOVE", *b"ABUSE", *b"ABYSS", *b"ACORN", *b"ACRID", 
	*b"ACTOR", *b"ACUTE", *b"ADAGE", *b"ADAPT", *b"ADEPT", *b"ADMIN", *b"ADMIT", *b"ADOBE", 
	*b"ADOPT", *b"ADORE", *b"ADORN", *b"ADULT", *b"AFFIX", *b"AFIRE", *b"AFOOT", *b"AFOUL", 
	*b"AFTER", *b"AGAIN", *b"AGAPE", *b"AGATE", *b"AGENT", *b"AGILE", *b"AGING", *b"AGLOW", 
	*b"AGONY", *b"AGREE", *b"AHEAD", *b"AIDER", *b"AISLE", *b"ALARM", *b"ALBUM", *b"ALERT", 
	*b"ALGAE", *b"ALIBI", *b"ALIEN", *b"ALIGN", *b"ALIKE", *b"ALIVE", *b"ALLAY", *b"ALLEY", 
	*b"ALLOT", *b"ALLOW", *b"ALLOY", *b"ALOFT", *b"ALONE", *b"ALONG", *b"ALOOF", *b"ALOUD", 
	*b"ALPHA", *b"ALTAR", *b"ALTER", *b"AMASS", *b"AMAZE", *b"AMBER", *b"AMBLE", *b"AMEND", 
	*b"AMISS", *b"AMITY", *b"AMONG", *b"AMPLE", *b"AMPLY", *b"AMUSE", *b"ANGEL", *b"ANGER", 
	*b"ANGLE", *b"ANGRY", *b"ANGST", *b"ANIME", *b"ANKLE", *b"ANNEX", *b"ANNOY", *b"ANNUL", 
	*b"ANODE", *b"ANTIC", *b"ANVIL", *b"AORTA", *b"APART", *b"APHID", *b"APING", *b"APNEA", 
	*b"APPLE", *b"APPLY", *b"APRON", *b"APTLY", *b"ARBOR", *b"ARDOR", *b"ARENA", *b"ARGUE", 
	*b"ARISE", *b"ARMOR", *b"AROMA", *b"AROSE", *b"ARRAY", *b"ARROW", *b"ARSON", *b"ARTSY", 
	*b"ASCOT", *b"ASHEN", *b"ASIDE", *b"ASKEW", *b"ASSAY", *b"ASSET", *b"ATOLL", *b"ATONE", 
	*b"ATTIC", *b"AUDIO", *b"AUDIT", *b"AUGUR", *b"AUNTY", *b"AVAIL", *b"AVERT", *b"AVIAN", 
	*b"AVOID", *b"AWAIT", *b"AWAKE", *b"AWARD", *b"AWARE", *b"AWASH", *b"AWFUL", *b"AWOKE", 
	*b"AXIAL", *b"AXIOM", *b"AXION", *b"AZURE", *b"BACON", *b"BADGE", *b"BADLY", *b"BAGEL", 
	*b"BAGGY", *b"BAKER", *b"BALER", *b"BALMY", *b"BANAL", *b"BANJO", *b"BARGE", *b"BARON", 
	*b"BASAL", *b"BASIC", *b"BASIL", *b"BASIN", *b"BASIS", *b"BASTE", *b"BATCH", *b"BATHE", 
	*b"BATON", *b"BATTY", *b"BAWDY", *b"BAYOU", *b"BEACH", *b"BEADY", *b"BEARD", *b"BEAST", 
	*b"BEECH", *b"BEEFY", *b"BEFIT", *b"BEGAN", *b"BEGAT", *b"BEGET", *b"BEGIN", *b"BEGUN", 
	*b"BEING", *b"BELCH", *b"BELIE", *b"BELLE", *b"BELLY", *b"BELOW", *b"BENCH", *b"BERET", 
	*b"BERRY", *b"BERTH", *b"BESET", *b"BETEL", *b"BEVEL", *b"BEZEL", *b"BIBLE", *b"BICEP", 
	*b"BIDDY", *b"BIGOT", *b"BILGE", *b"BILLY", *b"BINGE", *b"BINGO", *b"BIOME", *b"BIRCH", 
	*b"BIRTH", *b"BISON", *b"BITTY", *b"BLACK", *b"BLADE", *b"BLAME", *b"BLAND", *b"BLANK", 
	*b"BLARE", *b"BLAST", *b"BLAZE", *b"BLEAK", *b"BLEAT", *b"BLEED", *b"BLEEP", *b"BLEND", 
	*b"BLESS", *b"BLIMP", *b"BLIND", *b"BLINK", *b"BLISS", *b"BLITZ", *b"BLOAT", *b"BLOCK", 
	*b"BLOKE", *b"BLOND", *b"BLOOD", *b"BLOOM", *b"BLOWN", *b"BLUER", *b"BLUFF", *b"BLUNT", 
	*b"BLURB", *b"BLURT", *b"BLUSH", *b"BOARD", *b"BOAST", *b"BOBBY", *b"BONEY", *b"BONGO", 
	*b"BONUS", *b"BOOBY", *b"BOOST", *b"BOOTH", *b"BOOTY", *b"BOOZE", *b"BOOZY", *b"BORAX", 
	*b"BORNE", *b"BOSOM", *b"BOSSY", *b"BOTCH", *b"BOUGH", *b"BOULE", *b"BOUND", *b"BOWEL", 
	*b"BOXER", *b"BRACE", *b"BRAID", *b"BRAIN", *b"BRAKE", *b"BRAND", *b"BRASH", *b"BRASS", 
	*b"BRAVE", *b"BRAVO", *b"BRAWL", *b"BRAWN", *b"BREAD", *b"BREAK", *b"BREED", *b"BRIAR", 
	*b"BRIBE", *b"BRICK", *b"BRIDE", *b"BRIEF", *b"BRINE", *b"BRING", *b"BRINK", *b"BRINY", 
	*b"BRISK", *b"BROAD", *b"BROIL", *b"BROKE", *b"BROOD", *b"BROOK", *b"BROOM", *b"BROTH", 
	*b"BROWN", *b"BRUNT", *b"BRUSH", *b"BRUTE", *b"BUDDY", *b"BUDGE", *b"BUGGY", *b"BUGLE", 
	*b"BUILD", *b"BUILT", *b"BULGE", *b"BULKY", *b"BULLY", *b"BUNCH", *b"BUNNY", *b"BURLY", 
	*b"BURNT", *b"BURST", *b"BUSED", *b"BUSHY", *b"BUTCH", *b"BUTTE", *b"BUXOM", *b"BUYER", 
	*b"BYLAW", *b"CABAL", *b"CABBY", *b"CABIN", *b"CABLE", *b"CACAO", *b"CACHE", *b"CACTI", 
	*b"CADDY", *b"CADET", *b"CAGEY", *b"CAIRN", *b"CAMEL", *b"CAMEO", *b"CANAL", *b"CANDY", 
	*b"CANNY", *b"CANOE", *b"CANON", *b"CAPER", *b"CAPUT", *b"CARAT", *b"CARGO", *b"CAROL", 
	*b"CARRY", *b"CARVE", *b"CASTE", *b"CATCH", *b"CATER", *b"CATTY", *b"CAULK", *b"CAUSE", 
	*b"CAVIL", *b"CEASE", *b"CEDAR", *b"CELLO", *b"CHAFE", *b"CHAFF", *b"CHAIN", *b"CHAIR", 
	*b"CHALK", *b"CHAMP", *b"CHANT", *b"CHAOS", *b"CHARD", *b"CHARM", *b"CHART", *b"CHASE", 
	*b"CHASM", *b"CHEAP", *b"CHEAT", *b"CHECK", *b"CHEEK", *b"CHEER", *b"CHESS", *b"CHEST", 
	*b"CHICK", *b"CHIDE", *b"CHIEF", *b"CHILD", *b"CHILI", *b"CHILL", *b"CHIME", *b"CHINA", 
	*b"CHIRP", *b"CHOCK", *b"CHOIR", *b"CHOKE", *b"CHORD", *b"CHORE", *b"CHOSE", *b"CHUCK", 
	*b"CHUMP", *b"CHUNK", *b"CHURN", *b"CHUTE", *b"CIDER", *b"CIGAR", *b"CINCH", *b"CIRCA", 
	*b"CIVIC", *b"CIVIL", *b"CLACK", *b"CLAIM", *b"CLAMP", *b"CLANG", *b"CLANK", *b"CLASH", 
	*b"CLASP", *b"CLASS", *b"CLEAN", *b"CLEAR", *b"CLEAT", *b"CLEFT", *b"CLERK", *b"CLICK", 
	*b"CLIFF", *b"CLIMB", *b"CLING", *b"CLINK", *b"CLOAK", *b"CLOCK", *b"CLONE", *b"CLOSE", 
	*b"CLOTH", *b"CLOUD", *b"CLOUT", *b"CLOVE", *b"CLOWN", *b"CLUCK", *b"CLUED", *b"CLUMP", 
	*b"CLUNG", *b"COACH", *b"COAST", *b"COBRA", *b"COCOA", *b"COLON", *b"COLOR", *b"COMET", 
	*b"COMFY", *b"COMIC", *b"COMMA", *b"CONCH", *b"CONDO", *b"CONIC", *b"COPSE", *b"CORAL", 
	*b"CORER", *b"CORNY", *b"COUCH", *b"COUGH", *b"COULD", *b"COUNT", *b"COUPE", *b"COURT", 
	*b"COVEN", *b"COVER", *b"COVET", *b"COVEY", *b"COWER", *b"COYLY", *b"CRACK", *b"CRAFT", 
	*b"CRAMP", *b"CRANE", *b"CRANK", *b"CRASH", *b"CRASS", *b"CRATE", *b"CRAVE", *b"CRAWL", 
	*b"CRAZE", *b"CRAZY", *b"CREAK", *b"CREAM", *b"CREDO", *b"CREED", *b"CREEK", *b"CREEP", 
	*b"CREME", *b"CREPE", *b"CREPT", *b"CRESS", *b"CREST", *b"CRICK", *b"CRIED", *b"CRIER", 
	*b"CRIME", *b"CRIMP", *b"CRISP", *b"CROAK", *b"CROCK", *b"CRONE", *b"CRONY", *b"CROOK", 
	*b"CROSS", *b"CROUP", *b"CROWD", *b"CROWN", *b"CRUDE", *b"CRUEL", *b"CRUMB", *b"CRUMP", 
	*b"CRUSH", *b"CRUST", *b"CRYPT", *b"CUBIC", *b"CUMIN", *b"CURIO", *b"CURLY", *b"CURRY", 
	*b"CURSE", *b"CURVE", *b"CURVY", *b"CUTIE", *b"CYBER", *b"CYCLE", *b"CYNIC", *b"DADDY", 
	*b"DAILY", *b"DAIRY", *b"DAISY", *b"DALLY", *b"DANCE", *b"DANDY", *b"DATUM", *b"DAUNT", 
	*b"DEALT", *b"DEATH", *b"DEBAR", *b"DEBIT", *b"DEBUG", *b"DEBUT", *b"DECAL", *b"DECAY", 
	*b"DECOR", *b"DECOY", *b"DECRY", *b"DEFER", *b"DEIGN", *b"DEITY", *b"DELAY", *b"DELTA", 
	*b"DELVE", *b"DEMON", *b"DEMUR", *b"DENIM", *b"DENSE", *b"DEPOT", *b"DEPTH", *b"DERBY", 
	*b"DETER", *b"DETOX", *b"DEUCE", *b"DEVIL", *b"DIARY", *b"DICEY", *b"DIGIT", *b"DILLY", 
	*b"DIMLY", *b"DINER", *b"DINGO", *b"DINGY", *b"DIODE", *b"DIRGE", *b"DIRTY", *b"DISCO", 
	*b"DITCH", *b"DITTO", *b"DITTY", *b"DIVER", *b"DIZZY", *b"DODGE", *b"DODGY", *b"DOGMA", 
	*b"DOING", *b"DOLLY", *b"DONOR", *b"DONUT", *b"DOPEY", *b"DOUBT", *b"DOUGH", *b"DOWDY", 
	*b"DOWEL", *b"DOWNY", *b"DOWRY", *b"DOZEN", *b"DRAFT", *b"DRAIN", *b"DRAKE", *b"DRAMA", 
	*b"DRANK", *b"DRAPE", *b"DRAWL", *b"DRAWN", *b"DREAD", *b"DREAM", *b"DRESS", *b"DRIED", 
	*b"DRIER", *b"DRIFT", *b"DRILL", *b"DRINK", *b"DRIVE", *b"DROIT", *b"DROLL", *b"DRONE", 
	*b"DROOL", *b"DROOP", *b"DROSS", *b"DROVE", *b"DROWN", *b"DRUID", *b"DRUNK", *b"DRYER", 
	*b"DRYLY", *b"DUCHY", *b"DULLY", *b"DUMMY", *b"DUMPY", *b"DUNCE", *b"DUSKY", *b"DUSTY", 
	*b"DUTCH", *b"DUVET", *b"DWARF", *b"DWELL", *b"DWELT", *b"DYING", *b"EAGER", *b"EAGLE", 
	*b"EARLY", *b"EARTH", *b"EASEL", *b"EATEN", *b"EATER", *b"EBONY", *b"ECLAT", *b"EDICT", 
	*b"EDIFY", *b"EERIE", *b"EGRET", *b"EIGHT", *b"EJECT", *b"EKING", *b"ELATE", *b"ELBOW", 
	*b"ELDER", *b"ELECT", *b"ELEGY", *b"ELFIN", *b"ELIDE", *b"ELITE", *b"ELOPE", *b"ELUDE", 
	*b"EMAIL", *b"EMBED", *b"EMBER", *b"EMCEE", *b"EMPTY", *b"ENACT", *b"ENDOW", *b"ENEMA", 
	*b"ENEMY", *b"ENJOY", *b"ENNUI", *b"ENSUE", *b"ENTER", *b"ENTRY", *b"ENVOY", *b"EPOCH", 
	*b"EPOXY", *b"EQUAL", *b"EQUIP", *b"ERASE", *b"ERECT", *b"ERODE", *b"ERROR", *b"ERUPT", 
	*b"ESSAY", *b"ESTER", *b"ETHER", *b"ETHIC", *b"ETHOS", *b"ETUDE", *b"EVADE", *b"EVENT", 
	*b"EVERY", *b"EVICT", *b"EVOKE", *b"EXACT", *b"EXALT", *b"EXCEL", *b"EXERT", *b"EXILE", 
	*b"EXIST", *b"EXPEL", *b"EXTOL", *b"EXTRA", *b"EXULT", *b"EYING", *b"FABLE", *b"FACET", 
	*b"FAINT", *b"FAIRY", *b"FAITH", *b"FALSE", *b"FANCY", *b"FANNY", *b"FARCE", *b"FATAL", 
	*b"FATTY", *b"FAULT", *b"FAUNA", *b"FAVOR", *b"FEAST", *b"FECAL", *b"FEIGN", *b"FELLA", 
	*b"FELON", *b"FEMME", *b"FEMUR", *b"FENCE", *b"FERAL", *b"FERRY", *b"FETAL", *b"FETCH", 
	*b"FETID", *b"FETUS", *b"FEVER", *b"FEWER", *b"FIBER", *b"FICUS", *b"FIELD", *b"FIEND", 
	*b"FIERY", *b"FIFTH", *b"FIFTY", *b"FIGHT", *b"FILER", *b"FILET", *b"FILLY", *b"FILMY", 
	*b"FILTH", *b"FINAL", *b"FINCH", *b"FINER", *b"FIRST", *b"FISHY", *b"FIXER", *b"FIZZY", 
	*b"FJORD", *b"FLACK", *b"FLAIL", *b"FLAIR", *b"FLAKE", *b"FLAKY", *b"FLAME", *b"FLANK", 
	*b"FLARE", *b"FLASH", *b"FLASK", *b"FLECK", *b"FLEET", *b"FLESH", *b"FLICK", *b"FLIER", 
	*b"FLING", *b"FLINT", *b"FLIRT", *b"FLOAT", *b"FLOCK", *b"FLOOD", *b"FLOOR", *b"FLORA", 
	*b"FLOSS", *b"FLOUR", *b"FLOUT", *b"FLOWN", *b"FLUFF", *b"FLUID", *b"FLUKE", *b"FLUME", 
	*b"FLUNG", *b"FLUNK", *b"FLUSH", *b"FLUTE", *b"FLYER", *b"FOAMY", *b"FOCAL", *b"FOCUS", 
	*b"FOGGY", *b"FOIST", *b"FOLIO", *b"FOLLY", *b"FORAY", *b"FORCE", *b"FORGE", *b"FORGO", 
	*b"FORTE", *b"FORTH", *b"FORTY", *b"FORUM", *b"FOUND", *b"FOYER", *b"FRAIL", *b"FRAME", 
	*b"FRANK", *b"FRAUD", *b"FREAK", *b"FREED", *b"FREER", *b"FRESH", *b"FRIAR", *b"FRIED", 
	*b"FRILL", *b"FRISK", *b"FRITZ", *b"FROCK", *b"FROND", *b"FRONT", *b"FROST", *b"FROTH", 
	*b"FROWN", *b"FROZE", *b"FRUIT", *b"FUDGE", *b"FUGUE", *b"FULLY", *b"FUNGI", *b"FUNKY", 
	*b"FUNNY", *b"FUROR", *b"FURRY", *b"FUSSY", *b"FUZZY", *b"GAFFE", *b"GAILY", *b"GAMER", 
	*b"GAMMA", *b"GAMUT", *b"GASSY", *b"GAUDY", *b"GAUGE", *b"GAUNT", *b"GAUZE", *b"GAVEL", 
	*b"GAWKY", *b"GAYER", *b"GAYLY", *b"GAZER", *b"GECKO", *b"GEEKY", *b"GEESE", *b"GENIE", 
	*b"GENRE", *b"GHOST", *b"GHOUL", *b"GIANT", *b"GIDDY", *b"GIPSY", *b"GIRLY", *b"GIRTH", 
	*b"GIVEN", *b"GIVER", *b"GLADE", *b"GLAND", *b"GLARE", *b"GLASS", *b"GLAZE", *b"GLEAM", 
	*b"GLEAN", *b"GLIDE", *b"GLINT", *b"GLOAT", *b"GLOBE", *b"GLOOM", *b"GLORY", *b"GLOSS", 
	*b"GLOVE", *b"GLYPH", *b"GNASH", *b"GNOME", *b"GODLY", *b"GOING", *b"GOLEM", *b"GOLLY", 
	*b"GONAD", *b"GONER", *b"GOODY", *b"GOOEY", *b"GOOFY", *b"GOOSE", *b"GORGE", *b"GOUGE", 
	*b"GOURD", *b"GRACE", *b"GRADE", *b"GRAFT", *b"GRAIL", *b"GRAIN", *b"GRAND", *b"GRANT", 
	*b"GRAPE", *b"GRAPH", *b"GRASP", *b"GRASS", *b"GRATE", *b"GRAVE", *b"GRAVY", *b"GRAZE", 
	*b"GREAT", *b"GREED", *b"GREEN", *b"GREET", *b"GRIEF", *b"GRILL", *b"GRIME", *b"GRIMY", 
	*b"GRIND", *b"GRIPE", *b"GROAN", *b"GROIN", *b"GROOM", *b"GROPE", *b"GROSS", *b"GROUP", 
	*b"GROUT", *b"GROVE", *b"GROWL", *b"GROWN", *b"GRUEL", *b"GRUFF", *b"GRUNT", *b"GUARD", 
	*b"GUAVA", *b"GUESS", *b"GUEST", *b"GUIDE", *b"GUILD", *b"GUILE", *b"GUILT", *b"GUISE", 
	*b"GULCH", *b"GULLY", *b"GUMBO", *b"GUMMY", *b"GUPPY", *b"GUSTO", *b"GUSTY", *b"GYPSY", 
	*b"HABIT", *b"HAIRY", *b"HALVE", *b"HANDY", *b"HAPPY", *b"HARDY", *b"HAREM", *b"HARPY", 
	*b"HARRY", *b"HARSH", *b"HASTE", *b"HASTY", *b"HATCH", *b"HATER", *b"HAUNT", *b"HAUTE", 
	*b"HAVEN", *b"HAVOC", *b"HAZEL", *b"HEADY", *b"HEARD", *b"HEART", *b"HEATH", *b"HEAVE", 
	*b"HEAVY", *b"HEDGE", *b"HEFTY", *b"HEIST", *b"HELIX", *b"HELLO", *b"HENCE", *b"HERON", 
	*b"HILLY", *b"HINGE", *b"HIPPO", *b"HIPPY", *b"HITCH", *b"HOARD", *b"HOBBY", *b"HOIST", 
	*b"HOLLY", *b"HOMER", *b"HONEY", *b"HONOR", *b"HORDE", *b"HORNY", *b"HORSE", *b"HOTEL", 
	*b"HOTLY", *b"HOUND", *b"HOUSE", *b"HOVEL", *b"HOVER", *b"HOWDY", *b"HUMAN", *b"HUMID", 
	*b"HUMOR", *b"HUMPH", *b"HUMUS", *b"HUNCH", *b"HUNKY", *b"HURRY", *b"HUSKY", *b"HUSSY", 
	*b"HUTCH", *b"HYDRO", *b"HYENA", *b"HYMEN", *b"HYPER", *b"ICILY", *b"ICING", *b"IDEAL", 
	*b"IDIOM", *b"IDIOT", *b"IDLER", *b"IDYLL", *b"IGLOO", *b"ILIAC", *b"IMAGE", *b"IMBUE", 
	*b"IMPEL", *b"IMPLY", *b"INANE", *b"INBOX", *b"INCUR", *b"INDEX", *b"INEPT", *b"INERT", 
	*b"INFER", *b"INGOT", *b"INLAY", *b"INLET", *b"INNER", *b"INPUT", *b"INTER", *b"INTRO", 
	*b"IONIC", *b"IRATE", *b"IRONY", *b"ISLET", *b"ISSUE", *b"ITCHY", *b"IVORY", *b"JAUNT", 
	*b"JAZZY", *b"JELLY", *b"JERKY", *b"JETTY", *b"JEWEL", *b"JIFFY", *b"JOINT", *b"JOIST", 
	*b"JOKER", *b"JOLLY", *b"JOUST", *b"JUDGE", *b"JUICE", *b"JUICY", *b"JUMBO", *b"JUMPY", 
	*b"JUNTA", *b"JUNTO", *b"JUROR", *b"KAPPA", *b"KARMA", *b"KAYAK", *b"KEBAB", *b"KHAKI", 
	*b"KINKY", *b"KIOSK", *b"KITTY", *b"KNACK", *b"KNAVE", *b"KNEAD", *b"KNEED", *b"KNEEL", 
	*b"KNELT", *b"KNIFE", *b"KNOCK", *b"KNOLL", *b"KNOWN", *b"KOALA", *b"KRILL", *b"LABEL", 
	*b"LABOR", *b"LADEN", *b"LADLE", *b"LAGER", *b"LANCE", *b"LANKY", *b"LAPEL", *b"LAPSE", 
	*b"LARGE", *b"LARVA", *b"LASSO", *b"LATCH", *b"LATER", *b"LATHE", *b"LATTE", *b"LAUGH", 
	*b"LAYER", *b"LEACH", *b"LEAFY", *b"LEAKY", *b"LEANT", *b"LEAPT", *b"LEARN", *b"LEASE", 
	*b"LEASH", *b"LEAST", *b"LEAVE", *b"LEDGE", *b"LEECH", *b"LEERY", *b"LEFTY", *b"LEGAL", 
	*b"LEGGY", *b"LEMON", *b"LEMUR", *b"LEPER", *b"LEVEL", *b"LEVER", *b"LIBEL", *b"LIEGE", 
	*b"LIGHT", *b"LIKEN", *b"LILAC", *b"LIMBO", *b"LIMIT", *b"LINEN", *b"LINER", *b"LINGO", 
	*b"LIPID", *b"LITHE", *b"LIVER", *b"LIVID", *b"LLAMA", *b"LOAMY", *b"LOATH", *b"LOBBY", 
	*b"LOCAL", *b"LOCUS", *b"LODGE", *b"LOFTY", *b"LOGIC", *b"LOGIN", *b"LOOPY", *b"LOOSE", 
	*b"LORRY", *b"LOSER", *b"LOUSE", *b"LOUSY", *b"LOVER", *b"LOWER", *b"LOWLY", *b"LOYAL", 
	*b"LUCID", *b"LUCKY", *b"LUMEN", *b"LUMPY", *b"LUNAR", *b"LUNCH", *b"LUNGE", *b"LUPUS", 
	*b"LURCH", *b"LURID", *b"LUSTY", *b"LYING", *b"LYMPH", *b"LYRIC", *b"MACAW", *b"MACHO", 
	*b"MACRO", *b"MADAM", *b"MADLY", *b"MAFIA", *b"MAGIC", *b"MAGMA", *b"MAIZE", *b"MAJOR", 
	*b"MAKER", *b"MAMBO", *b"MAMMA", *b"MAMMY", *b"MANGA", *b"MANGE", *b"MANGO", *b"MANGY", 
	*b"MANIA", *b"MANIC", *b"MANLY", *b"MANOR", *b"MAPLE", *b"MARCH", *b"MARRY", *b"MARSH", 
	*b"MASON", *b"MASSE", *b"MATCH", *b"MATEY", *b"MAUVE", *b"MAXIM", *b"MAYBE", *b"MAYOR", 
	*b"MEALY", *b"MEANT", *b"MEATY", *b"MECCA", *b"MEDAL", *b"MEDIA", *b"MEDIC", *b"MELEE", 
	*b"MELON", *b"MERCY", *b"MERGE", *b"MERIT", *b"MERRY", *b"METAL", *b"METER", *b"METRO", 
	*b"MICRO", *b"MIDGE", *b"MIDST", *b"MIGHT", *b"MILKY", *b"MIMIC", *b"MINCE", *b"MINER", 
	*b"MINIM", *b"MINOR", *b"MINTY", *b"MINUS", *b"MIRTH", *b"MISER", *b"MISSY", *b"MOCHA", 
	*b"MODAL", *b"MODEL", *b"MODEM", *b"MOGUL", *b"MOIST", *b"MOLAR", *b"MOLDY", *b"MONEY", 
	*b"MONTH", *b"MOODY", *b"MOOSE", *b"MORAL", *b"MORON", *b"MORPH", *b"MOSSY", *b"MOTEL", 
	*b"MOTIF", *b"MOTOR", *b"MOTTO", *b"MOULT", *b"MOUND", *b"MOUNT", *b"MOURN", *b"MOUSE", 
	*b"MOUTH", *b"MOVER", *b"MOVIE", *b"MOWER", *b"MUCKY", *b"MUCUS", *b"MUDDY", *b"MULCH", 
	*b"MUMMY", *b"MUNCH", *b"MURAL", *b"MURKY", *b"MUSHY", *b"MUSIC", *b"MUSKY", *b"MUSTY", 
	*b"MYRRH", *b"NADIR", *b"NAIVE", *b"NANNY", *b"NASAL", *b"NASTY", *b"NATAL", *b"NAVAL", 
	*b"NAVEL", *b"NEEDY", *b"NEIGH", *b"NERDY", *b"NERVE", *b"NEVER", *b"NEWER", *b"NEWLY", 
	*b"NICER", *b"NICHE", *b"NIECE", *b"NIGHT", *b"NINJA", *b"NINNY", *b"NINTH", *b"NOBLE", 
	*b"NOBLY", *b"NOISE", *b"NOISY", *b"NOMAD", *b"NOOSE", *b"NORTH", *b"NOSEY", *b"NOTCH", 
	*b"NOVEL", *b"NUDGE", *b"NURSE", *b"NUTTY", *b"NYLON", *b"NYMPH", *b"OAKEN", *b"OBESE", 
	*b"OCCUR", *b"OCEAN", *b"OCTAL", *b"OCTET", *b"ODDER", *b"ODDLY", *b"OFFAL", *b"OFFER", 
	*b"OFTEN", *b"OLDEN", *b"OLDER", *b"OLIVE", *b"OMBRE", *b"OMEGA", *b"ONION", *b"ONSET", 
	*b"OPERA", *b"OPINE", *b"OPIUM", *b"OPTIC", *b"ORBIT", *b"ORDER", *b"ORGAN", *b"OTHER", 
	*b"OTTER", *b"OUGHT", *b"OUNCE", *b"OUTDO", *b"OUTER", *b"OUTGO", *b"OVARY", *b"OVATE", 
	*b"OVERT", *b"OVINE", *b"OVOID", *b"OWING", *b"OWNER", *b"OXIDE", *b"OZONE", *b"PADDY", 
	*b"PAGAN", *b"PAINT", *b"PALER", *b"PALSY", *b"PANEL", *b"PANIC", *b"PANSY", *b"PAPAL", 
	*b"PAPER", *b"PARER", *b"PARKA", *b"PARRY", *b"PARSE", *b"PARTY", *b"PASTA", *b"PASTE", 
	*b"PASTY", *b"PATCH", *b"PATIO", *b"PATSY", *b"PATTY", *b"PAUSE", *b"PAYEE", *b"PAYER", 
	*b"PEACE", *b"PEACH", *b"PEARL", *b"PECAN", *b"PEDAL", *b"PENAL", *b"PENCE", *b"PENNE", 
	*b"PENNY", *b"PERCH", *b"PERIL", *b"PERKY", *b"PESKY", *b"PESTO", *b"PETAL", *b"PETTY", 
	*b"PHASE", *b"PHONE", *b"PHONY", *b"PHOTO", *b"PIANO", *b"PICKY", *b"PIECE", *b"PIETY", 
	*b"PIGGY", *b"PILOT", *b"PINCH", *b"PINEY", *b"PINKY", *b"PINTO", *b"PIPER", *b"PIQUE", 
	*b"PITCH", *b"PITHY", *b"PIVOT", *b"PIXEL", *b"PIXIE", *b"PIZZA", *b"PLACE", *b"PLAID", 
	*b"PLAIN", *b"PLAIT", *b"PLANE", *b"PLANK", *b"PLANT", *b"PLATE", *b"PLAZA", *b"PLEAD", 
	*b"PLEAT", *b"PLIED", *b"PLIER", *b"PLUCK", *b"PLUMB", *b"PLUME", *b"PLUMP", *b"PLUNK", 
	*b"PLUSH", *b"POESY", *b"POINT", *b"POISE", *b"POKER", *b"POLAR", *b"POLKA", *b"POLYP", 
	*b"POOCH", *b"POPPY", *b"PORCH", *b"POSER", *b"POSIT", *b"POSSE", *b"POUCH", *b"POUND", 
	*b"POUTY", *b"POWER", *b"PRANK", *b"PRAWN", *b"PREEN", *b"PRESS", *b"PRICE", *b"PRICK", 
	*b"PRIDE", *b"PRIED", *b"PRIME", *b"PRIMO", *b"PRINT", *b"PRIOR", *b"PRISM", *b"PRIVY", 
	*b"PRIZE", *b"PROBE", *b"PRONE", *b"PRONG", *b"PROOF", *b"PROSE", *b"PROUD", *b"PROVE", 
	*b"PROWL", *b"PROXY", *b"PRUDE", *b"PRUNE", *b"PSALM", *b"PUBIC", *b"PUDGY", *b"PUFFY", 
	*b"PULPY", *b"PULSE", *b"PUNCH", *b"PUPIL", *b"PUPPY", *b"PUREE", *b"PURER", *b"PURGE", 
	*b"PURSE", *b"PUSHY", *b"PUTTY", *b"PYGMY", *b"QUACK", *b"QUAIL", *b"QUAKE", *b"QUALM", 
	*b"QUARK", *b"QUART", *b"QUASH", *b"QUASI", *b"QUEEN", *b"QUEER", *b"QUELL", *b"QUERY", 
	*b"QUEST", *b"QUEUE", *b"QUICK", *b"QUIET", *b"QUILL", *b"QUILT", *b"QUIRK", *b"QUITE", 
	*b"QUOTA", *b"QUOTE", *b"QUOTH", *b"RABBI", *b"RABID", *b"RACER", *b"RADAR", *b"RADII", 
	*b"RADIO", *b"RAINY", *b"RAISE", *b"RAJAH", *b"RALLY", *b"RALPH", *b"RAMEN", *b"RANCH", 
	*b"RANDY", *b"RANGE", *b"RAPID", *b"RARER", *b"RASPY", *b"RATIO", *b"RATTY", *b"RAVEN", 
	*b"RAYON", *b"RAZOR", *b"REACH", *b"REACT", *b"READY", *b"REALM", *b"REARM", *b"REBAR", 
	*b"REBEL", *b"REBUS", *b"REBUT", *b"RECAP", *b"RECUR", *b"RECUT", *b"REEDY", *b"REFER", 
	*b"REFIT", *b"REGAL", *b"REHAB", *b"REIGN", *b"RELAX", *b"RELAY", *b"RELIC", *b"REMIT", 
	*b"RENAL", *b"RENEW", *b"REPAY", *b"REPEL", *b"REPLY", *b"RERUN", *b"RESET", *b"RESIN", 
	*b"RETCH", *b"RETRO", *b"RETRY", *b"REUSE", *b"REVEL", *b"REVUE", *b"RHINO", *b"RHYME", 
	*b"RIDER", *b"RIDGE", *b"RIFLE", *b"RIGHT", *b"RIGID", *b"RIGOR", *b"RINSE", *b"RIPEN", 
	*b"RIPER", *b"RISEN", *b"RISER", *b"RISKY", *b"RIVAL", *b"RIVER", *b"RIVET", *b"ROACH", 
	*b"ROAST", *b"ROBIN", *b"ROBOT", *b"ROCKY", *b"RODEO", *b"ROGER", *b"ROGUE", *b"ROOMY", 
	*b"ROOST", *b"ROTOR", *b"ROUGE", *b"ROUGH", *b"ROUND", *b"ROUSE", *b"ROUTE", *b"ROVER", 
	*b"ROWDY", *b"ROWER", *b"ROYAL", *b"RUDDY", *b"RUDER", *b"RUGBY", *b"RULER", *b"RUMBA", 
	*b"RUMOR", *b"RUPEE", *b"RURAL", *b"RUSTY", *b"SADLY", *b"SAFER", *b"SAINT", *b"SALAD", 
	*b"SALLY", *b"SALON", *b"SALSA", *b"SALTY", *b"SALVE", *b"SALVO", *b"SANDY", *b"SANER", 
	*b"SAPPY", *b"SASSY", *b"SATIN", *b"SATYR", *b"SAUCE", *b"SAUCY", *b"SAUNA", *b"SAUTE", 
	*b"SAVOR", *b"SAVOY", *b"SAVVY", *b"SCALD", *b"SCALE", *b"SCALP", *b"SCALY", *b"SCAMP", 
	*b"SCANT", *b"SCARE", *b"SCARF", *b"SCARY", *b"SCENE", *b"SCENT", *b"SCION", *b"SCOFF", 
	*b"SCOLD", *b"SCONE", *b"SCOOP", *b"SCOPE", *b"SCORE", *b"SCORN", *b"SCOUR", *b"SCOUT", 
	*b"SCOWL", *b"SCRAM", *b"SCRAP", *b"SCREE", *b"SCREW", *b"SCRUB", *b"SCRUM", *b"SCUBA", 
	*b"SEDAN", *b"SEEDY", *b"SEGUE", *b"SEIZE", *b"SEMEN", *b"SENSE", *b"SEPIA", *b"SERIF", 
	*b"SERUM", *b"SERVE", *b"SETUP", *b"SEVEN", *b"SEVER", *b"SEWER", *b"SHACK", *b"SHADE", 
	*b"SHADY", *b"SHAFT", *b"SHAKE", *b"SHAKY", *b"SHALE", *b"SHALL", *b"SHALT", *b"SHAME", 
	*b"SHANK", *b"SHAPE", *b"SHARD", *b"SHARE", *b"SHARK", *b"SHARP", *b"SHAVE", *b"SHAWL", 
	*b"SHEAR", *b"SHEEN", *b"SHEEP", *b"SHEER", *b"SHEET", *b"SHEIK", *b"SHELF", *b"SHELL", 
	*b"SHIED", *b"SHIFT", *b"SHINE", *b"SHINY", *b"SHIRE", *b"SHIRK", *b"SHIRT", *b"SHOAL", 
	*b"SHOCK", *b"SHONE", *b"SHOOK", *b"SHOOT", *b"SHORE", *b"SHORN", *b"SHORT", *b"SHOUT", 
	*b"SHOVE", *b"SHOWN", *b"SHOWY", *b"SHREW", *b"SHRUB", *b"SHRUG", *b"SHUCK", *b"SHUNT", 
	*b"SHUSH", *b"SHYLY", *b"SIEGE", *b"SIEVE", *b"SIGHT", *b"SIGMA", *b"SILKY", *b"SILLY", 
	*b"SINCE", *b"SINEW", *b"SINGE", *b"SIREN", *b"SISSY", *b"SIXTH", *b"SIXTY", *b"SKATE", 
	*b"SKIER", *b"SKIFF", *b"SKILL", *b"SKIMP", *b"SKIRT", *b"SKULK", *b"SKULL", *b"SKUNK", 
	*b"SLACK", *b"SLAIN", *b"SLANG", *b"SLANT", *b"SLASH", *b"SLATE", *b"SLEEK", *b"SLEEP", 
	*b"SLEET", *b"SLEPT", *b"SLICE", *b"SLICK", *b"SLIDE", *b"SLIME", *b"SLIMY", *b"SLING", 
	*b"SLINK", *b"SLOOP", *b"SLOPE", *b"SLOSH", *b"SLOTH", *b"SLUMP", *b"SLUNG", *b"SLUNK", 
	*b"SLURP", *b"SLUSH", *b"SLYLY", *b"SMACK", *b"SMALL", *b"SMART", *b"SMASH", *b"SMEAR", 
	*b"SMELL", *b"SMELT", *b"SMILE", *b"SMIRK", *b"SMITE", *b"SMITH", *b"SMOCK", *b"SMOKE", 
	*b"SMOKY", *b"SMOTE", *b"SNACK", *b"SNAIL", *b"SNAKE", *b"SNAKY", *b"SNARE", *b"SNARL", 
	*b"SNEAK", *b"SNEER", *b"SNIDE", *b"SNIFF", *b"SNIPE", *b"SNOOP", *b"SNORE", *b"SNORT", 
	*b"SNOUT", *b"SNOWY", *b"SNUCK", *b"SNUFF", *b"SOAPY", *b"SOBER", *b"SOGGY", *b"SOLAR", 
	*b"SOLID", *b"SOLVE", *b"SONAR", *b"SONIC", *b"SOOTH", *b"SOOTY", *b"SORRY", *b"SOUND", 
	*b"SOUTH", *b"SOWER", *b"SPACE", *b"SPADE", *b"SPANK", *b"SPARE", *b"SPARK", *b"SPASM", 
	*b"SPAWN", *b"SPEAK", *b"SPEAR", *b"SPECK", *b"SPEED", *b"SPELL", *b"SPELT", *b"SPEND", 
	*b"SPENT", *b"SPERM", *b"SPICE", *b"SPICY", *b"SPIED", *b"SPIEL", *b"SPIKE", *b"SPIKY", 
	*b"SPILL", *b"SPILT", *b"SPINE", *b"SPINY", *b"SPIRE", *b"SPITE", *b"SPLAT", *b"SPLIT", 
	*b"SPOIL", *b"SPOKE", *b"SPOOF", *b"SPOOK", *b"SPOOL", *b"SPOON", *b"SPORE", *b"SPORT", 
	*b"SPOUT", *b"SPRAY", *b"SPREE", *b"SPRIG", *b"SPUNK", *b"SPURN", *b"SPURT", *b"SQUAD", 
	*b"SQUAT", *b"SQUIB", *b"STACK", *b"STAFF", *b"STAGE", *b"STAID", *b"STAIN", *b"STAIR", 
	*b"STAKE", *b"STALE", *b"STALK", *b"STALL", *b"STAMP", *b"STAND", *b"STANK", *b"STARE", 
	*b"STARK", *b"START", *b"STASH", *b"STATE", *b"STAVE", *b"STEAD", *b"STEAK", *b"STEAL", 
	*b"STEAM", *b"STEED", *b"STEEL", *b"STEEP", *b"STEER", *b"STEIN", *b"STERN", *b"STICK", 
	*b"STIFF", *b"STILL", *b"STILT", *b"STING", *b"STINK", *b"STINT", *b"STOCK", *b"STOIC", 
	*b"STOKE", *b"STOLE", *b"STOMP", *b"STONE", *b"STONY", *b"STOOD", *b"STOOL", *b"STOOP", 
	*b"STORE", *b"STORK", *b"STORM", *b"STORY", *b"STOUT", *b"STOVE", *b"STRAP", *b"STRAW", 
	*b"STRAY", *b"STRIP", *b"STRUT", *b"STUCK", *b"STUDY", *b"STUFF", *b"STUMP", *b"STUNG", 
	*b"STUNK", *b"STUNT", *b"STYLE", *b"SUAVE", *b"SUGAR", *b"SUING", *b"SUITE", *b"SULKY", 
	*b"SULLY", *b"SUMAC", *b"SUNNY", *b"SUPER", *b"SURER", *b"SURGE", *b"SURLY", *b"SUSHI", 
	*b"SWAMI", *b"SWAMP", *b"SWARM", *b"SWASH", *b"SWATH", *b"SWEAR", *b"SWEAT", *b"SWEEP", 
	*b"SWEET", *b"SWELL", *b"SWEPT", *b"SWIFT", *b"SWILL", *b"SWINE", *b"SWING", *b"SWIRL", 
	*b"SWISH", *b"SWOON", *b"SWOOP", *b"SWORD", *b"SWORE", *b"SWORN", *b"SWUNG", *b"SYNOD", 
	*b"SYRUP", *b"TABBY", *b"TABLE", *b"TABOO", *b"TACIT", *b"TACKY", *b"TAFFY", *b"TAINT", 
	*b"TAKEN", *b"TAKER", *b"TALLY", *b"TALON", *b"TAMER", *b"TANGO", *b"TANGY", *b"TAPER", 
	*b"TAPIR", *b"TARDY", *b"TAROT", *b"TASTE", *b"TASTY", *b"TATTY", *b"TAUNT", *b"TAWNY", 
	*b"TEACH", *b"TEARY", *b"TEASE", *b"TEDDY", *b"TEETH", *b"TEMPO", *b"TENET", *b"TENOR", 
	*b"TENSE", *b"TENTH", *b"TEPEE", *b"TEPID", *b"TERRA", *b"TERSE", *b"TESTY", *b"THANK", 
	*b"THEFT", *b"THEIR", *b"THEME", *b"THERE", *b"THESE", *b"THETA", *b"THICK", *b"THIEF", 
	*b"THIGH", *b"THING", *b"THINK", *b"THIRD", *b"THONG", *b"THORN", *b"THOSE", *b"THREE", 
	*b"THREW", *b"THROB", *b"THROW", *b"THRUM", *b"THUMB", *b"THUMP", *b"THYME", *b"TIARA", 
	*b"TIBIA", *b"TIDAL", *b"TIGER", *b"TIGHT", *b"TILDE", *b"TIMER", *b"TIMID", *b"TIPSY", 
	*b"TITAN", *b"TITHE", *b"TITLE", *b"TOAST", *b"TODAY", *b"TODDY", *b"TOKEN", *b"TONAL", 
	*b"TONGA", *b"TONIC", *b"TOOTH", *b"TOPAZ", *b"TOPIC", *b"TORCH", *b"TORSO", *b"TORUS", 
	*b"TOTAL", *b"TOTEM", *b"TOUCH", *b"TOUGH", *b"TOWEL", *b"TOWER", *b"TOXIC", *b"TOXIN", 
	*b"TRACE", *b"TRACK", *b"TRACT", *b"TRADE", *b"TRAIL", *b"TRAIN", *b"TRAIT", *b"TRAMP", 
	*b"TRASH", *b"TRAWL", *b"TREAD", *b"TREAT", *b"TREND", *b"TRIAD", *b"TRIAL", *b"TRIBE", 
	*b"TRICE", *b"TRICK", *b"TRIED", *b"TRIPE", *b"TRITE", *b"TROLL", *b"TROOP", *b"TROPE", 
	*b"TROUT", *b"TROVE", *b"TRUCE", *b"TRUCK", *b"TRUER", *b"TRULY", *b"TRUMP", *b"TRUNK", 
	*b"TRUSS", *b"TRUST", *b"TRUTH", *b"TRYST", *b"TUBAL", *b"TUBER", *b"TULIP", *b"TULLE", 
	*b"TUMOR", *b"TUNIC", *b"TURBO", *b"TUTOR", *b"TWANG", *b"TWEAK", *b"TWEED", *b"TWEET", 
	*b"TWICE", *b"TWINE", *b"TWIRL", *b"TWIST", *b"TWIXT", *b"TYING", *b"UDDER", *b"ULCER", 
	*b"ULTRA", *b"UMBRA", *b"UNCLE", *b"UNCUT", *b"UNDER", *b"UNDID", *b"UNDUE", *b"UNFED", 
	*b"UNFIT", *b"UNIFY", *b"UNION", *b"UNITE", *b"UNITY", *b"UNLIT", *b"UNMET", *b"UNSET", 
	*b"UNTIE", *b"UNTIL", *b"UNWED", *b"UNZIP", *b"UPPER", *b"UPSET", *b"URBAN", *b"URINE", 
	*b"USAGE", *b"USHER", *b"USING", *b"USUAL", *b"USURP", *b"UTILE", *b"UTTER", *b"VAGUE", 
	*b"VALET", *b"VALID", *b"VALOR", *b"VALUE", *b"VALVE", *b"VAPID", *b"VAPOR", *b"VAULT", 
	*b"VAUNT", *b"VEGAN", *b"VENOM", *b"VENUE", *b"VERGE", *b"VERSE", *b"VERSO", *b"VERVE", 
	*b"VICAR", *b"VIDEO", *b"VIGIL", *b"VIGOR", *b"VILLA", *b"VINYL", *b"VIOLA", *b"VIPER", 
	*b"VIRAL", *b"VIRUS", *b"VISIT", *b"VISOR", *b"VISTA", *b"VITAL", *b"VIVID", *b"VIXEN", 
	*b"VOCAL", *b"VODKA", *b"VOGUE", *b"VOICE", *b"VOILA", *b"VOMIT", *b"VOTER", *b"VOUCH", 
	*b"VOWEL", *b"VYING", *b"WACKY", *b"WAFER", *b"WAGER", *b"WAGON", *b"WAIST", *b"WAIVE", 
	*b"WALTZ", *b"WARTY", *b"WASTE", *b"WATCH", *b"WATER", *b"WAVER", *b"WAXEN", *b"WEARY", 
	*b"WEAVE", *b"WEDGE", *b"WEEDY", *b"WEIGH", *b"WEIRD", *b"WELCH", *b"WELSH", *b"WHACK", 
	*b"WHALE", *b"WHARF", *b"WHEAT", *b"WHEEL", *b"WHELP", *b"WHERE", *b"WHICH", *b"WHIFF", 
	*b"WHILE", *b"WHINE", *b"WHINY", *b"WHIRL", *b"WHISK", *b"WHITE", *b"WHOLE", *b"WHOOP", 
	*b"WHOSE", *b"WIDEN", *b"WIDER", *b"WIDOW", *b"WIDTH", *b"WIELD", *b"WIGHT", *b"WILLY", 
	*b"WIMPY", *b"WINCE", *b"WINCH", *b"WINDY", *b"WISER", *b"WISPY", *b"WITCH", *b"WITTY", 
	*b"WOKEN", *b"WOMAN", *b"WOMEN", *b"WOODY", *b"WOOER", *b"WOOLY", *b"WOOZY", *b"WORDY", 
	*b"WORLD", *b"WORRY", *b"WORSE", *b"WORST", *b"WORTH", *b"WOULD", *b"WOUND", *b"WOVEN", 
	*b"WRACK", *b"WRATH", *b"WREAK", *b"WRECK", *b"WREST", *b"WRING", *b"WRIST", *b"WRITE", 
	*b"WRONG", *b"WROTE", *b"WRUNG", *b"WRYLY", *b"YACHT", *b"YEARN", *b"YEAST", *b"YIELD", 
	*b"YOUNG", *b"YOUTH", *b"ZEBRA", *b"ZESTY", *b"ZONAL"
];

pub const WORDLE_ANSWER_NUMBERS: [i32; NUM_WORDLE_ANSWERS] = [
	110, 26, 18, 208, 1671, 2044, 1037, 1705, 1606, 1329, 702, 1851, 1455, 126, 677, 1127, 1648, 190, 1072, 2258, 1190, 1627, 463, 40, 964, 1105, 1083, 2281, 2013, 1468, 1196, 1218, 1348, 1479, 383, 102, 1598, 1221, 535, 1264, 1024, 70, 258, 826, 1632, 1495, 339, 1537, 1021, 2189, 413, 1113, 458, 866, 2092, 883, 2072, 273, 2014, 231, 112, 1235, 1779, 502, 451, 137, 1482, 1043, 855, 2007, 999, 1136, 1836, 1362, 1017, 302, 769, 1025, 1818, 2164, 623, 395, 1099, 885, 1747, 1621, 1291, 651, 1873, 2116, 2145, 2280, 1665, 397, 678, 1891, 589, 1167, 362, 499, 804, 611, 989, 73, 534, 1701, 241, 1226, 1094, 1748, 633, 2284, 1031, 1078, 192, 310, 843, 341, 345, 360, 2228, 1563, 91, 2308, 926, 1206, 538, 1944, 1956, 2208, 4, 649, 1172, 1856, 368, 683, 1916, 1684, 2084, 834, 1899, 321, 146, 1681, 1232, 508, 1850, 1570, 201, 2083, 823, 1842, 2243, 100, 1810, 801, 1634, 2036, 2136, 925, 183, 29, 1401, 344, 1053, 371, 1225, 1490, 867, 1383, 1676, 1721, 1166, 1188, 1622, 833, 332, 118, 1846, 1985, 58, 1388, 17, 1962, 1929, 386, 772, 830, 1287, 2085, 1216, 1801, 2063, 2235, 897, 1552, 1832, 1389, 82, 687, 1753, 2033, 1340, 295, 2046, 2238, 389, 1011, 1587, 572, 1324, 1293, 1442, 74, 2170, 1582, 809, 1312, 1476, 723, 869, 1382, 1197, 904, 250, 815, 1605, 845, 363, 825, 406, 1182, 1048, 139, 5, 1896, 1908, 2287, 2041, 832, 1133, 54, 197, 1736, 1889, 450, 130, 698, 2004, 887, 1707, 1637, 472, 715, 1675, 639, 1202, 2091, 839, 795, 184, 1795, 1391, 929, 968, 810, 1901, 684, 1001, 172, 1308, 135, 64, 1415, 1486, 1327, 259, 164, 369, 1670, 464, 1683, 1229, 1781, 1007, 1712, 1475, 2148, 2197, 2024, 1702, 1932, 796, 750, 412, 1434, 1138, 647, 1789, 1974, 1198, 2143, 1186, 1377, 914, 1360, 1812, 1026, 2304, 1798, 981, 1068, 1176, 2052, 2125, 953, 1900, 364, 1311, 1616, 756, 1827, 2027, 1660, 1815, 1646, 1505, 799, 323, 566, 1173, 1200, 1686, 2009, 305, 1365, 495, 585, 1058, 483, 270, 2273, 242, 1262, 939, 1751, 1296, 1870, 1310, 670, 2087, 1371, 1436, 182, 253, 567, 1685, 440, 1783, 626, 1698, 1316, 166, 927, 301, 1680, 2119, 278, 1503, 1738, 436, 963, 1558, 150, 2132, 736, 2187, 1881, 892, 254, 800, 1194, 609, 1156, 708, 298, 1574, 455, 1341, 0, 402, 1977, 96, 862, 1759, 528, 1791, 2163, 2001, 1603, 899, 446, 735, 1337, 1547, 1886, 1656, 134, 774, 1653, 417, 727, 1566, 160, 2059, 2054, 260, 2051, 1554, 1081, 432, 46, 1056, 2233, 724, 600, 140, 1809, 1529, 25, 1760, 122, 1255, 1055, 291, 1212, 2257, 175, 1027, 1144, 2010, 98, 637, 921, 224, 545, 2042, 1814, 1910, 865, 568, 1728, 1930, 409, 2139, 1766, 407, 1703, 203, 1042, 67, 45, 524, 1638, 186, 41, 347, 619, 1171, 884, 642, 1057, 1345, 1591, 343, 710, 1102, 916, 2131, 1426, 1874, 218, 1674, 33, 666, 2217, 2133, 734, 1006, 726, 803, 1033, 1664, 1441, 1458, 941, 1987, 23, 1302, 1641, 1303, 1561, 1295, 2081, 1689, 2140, 2026, 821, 1104, 713, 240, 532, 1918, 1004, 911, 808, 1555, 475, 1137, 2205, 1965, 21, 1657, 1241, 1872, 1594, 1418, 730, 1488, 1745, 2201, 1423, 1313, 1282, 896, 75, 331, 1619, 936, 488, 1199, 279, 351, 597, 550, 1372, 2029, 1958, 1124, 1768, 44, 1041, 984, 632, 997, 1797, 621, 1297, 2011, 1050, 1773, 617, 1294, 879, 1710, 244, 1829, 1421, 767, 872, 359, 987, 2129, 453, 942, 753, 636, 2159, 116, 129, 1289, 63, 1466, 1805, 1126, 2068, 910, 1677, 1358, 504, 1772, 2150, 1432, 1860, 1799, 206, 2165, 2057, 374, 1905, 1300, 1214, 650, 1161, 2117, 2126, 1063, 1123, 1125, 141, 2099, 1474, 758, 888, 1709, 1447, 119, 1343, 11, 1223, 851, 793, 1752, 644, 1573, 958, 1394, 783, 711, 1740, 2288, 779, 659, 594, 378, 873, 2275, 2100, 1539, 1457, 233, 933, 1497, 1739, 1317, 657, 401, 712, 2167, 648, 1350, 1093, 1353, 1082, 2043, 124, 1478, 478, 816, 877, 962, 1735, 1325, 170, 280, 481, 564, 1496, 1931, 131, 71, 601, 105, 1258, 1160, 1659, 1485, 1975, 7, 876, 976, 934, 641, 2127, 701, 951, 1982, 686, 486, 1848, 1487, 2256, 168, 2215, 1565, 1557, 1368, 605, 2263, 1319, 992, 2302, 326, 1719, 966, 492, 2177, 207, 1437, 1018, 19, 2289, 2195, 1514, 2186, 1862, 2254, 191, 1823, 1261, 794, 2303, 1843, 288, 751, 2199, 381, 1314, 1333, 2069, 1714, 1882, 2090, 969, 1551, 607, 1996, 592, 1077, 153, 56, 1158, 143, 1980, 106, 2306, 1926, 303, 1871, 2097, 1777, 1251, 1971, 1571, 2037, 2194, 520, 53, 76, 732, 128, 860, 1210, 358, 394, 353, 482, 1411, 31, 1370, 494, 745, 382, 818, 1195, 68, 2020, 1720, 2272, 1429, 764, 1793, 6, 267, 493, 1645, 2173, 514, 292, 1593, 97, 316, 2188, 55, 1281, 1904, 282, 304, 1859, 234, 1994, 1650, 1015, 1467, 689, 22, 1002, 2022, 1855, 831, 778, 1061, 1115, 79, 1887, 350, 653, 2112, 1988, 2185, 1456, 2078, 439, 1792, 555, 919, 931, 2181, 1623, 1266, 2290, 335, 93, 1464, 1409, 149, 2137, 1080, 435, 943, 375, 1961, 805, 1483, 325, 920, 2048, 836, 542, 691, 1662, 1716, 1953, 1355, 2039, 355, 579, 1285, 1523, 1964, 1230, 334, 2071, 1984, 418, 1152, 971, 367, 859, 352, 461, 1960, 838, 510, 2096, 739, 518, 2229, 87, 2192, 1062, 86, 652, 2089, 2166, 357, 204, 133, 2196, 616, 15, 2034, 2058, 1981, 2082, 1897, 906, 1754, 2031, 1419, 462, 521, 1305, 1059, 136, 1945, 1697, 85, 1049, 1347, 167, 2025, 584, 187, 582, 142, 1109, 1277, 1943, 144, 1470, 489, 127, 1298, 423, 1257, 1088, 2245, 2183, 1682, 975, 1644, 109, 562, 1530, 2060, 516, 441, 1695, 2178, 2120, 663, 563, 1774, 1706, 318, 1067, 615, 2179, 908, 947, 1651, 2291, 1688, 1596, 2292, 113, 681, 1369, 2239, 938, 1278, 856, 1128, 1817, 630, 10, 1998, 1512, 2038, 1502, 311, 32, 2251, 2221, 151, 1663, 338, 1430, 512, 1111, 261, 1489, 980, 1417, 320, 591, 1398, 1299, 765, 1976, 1545, 901, 2232, 697, 786, 1364, 477, 569, 656, 235, 3, 1531, 1542, 420, 2103, 967, 1385, 376, 2293, 1806, 1454, 114, 1600, 741, 898, 1491, 645, 1162, 917, 1449, 1180, 2124, 1877, 2207, 935, 522, 1979, 546, 722, 501, 309, 1148, 1602, 707, 2216, 982, 366, 442, 2242, 480, 846, 433, 95, 1405, 1679, 59, 123, 1713, 1097, 1381, 868, 1451, 606, 1509, 1185, 874, 858, 1245, 2282, 1922, 728, 1203, 1938, 2203, 1130, 1604, 1361, 13, 2074, 108, 421, 2017, 814, 714, 1592, 1224, 2160, 1744, 754, 1869, 1029, 2255, 219, 2152, 1342, 1803, 419, 94, 655, 1764, 1406, 643, 709, 83, 264, 1022, 315, 1515, 1413, 847, 2008, 2141, 902, 817, 2230, 886, 583, 1687, 1835, 1252, 1577, 1553, 1259, 469, 1894, 2053, 445, 576, 1718, 1933, 2109, 1288, 1787, 985, 2122, 497, 2294, 226, 1959, 379, 1861, 1629, 51, 1560, 1292, 1699, 1711, 390, 1950, 782, 2247, 669, 719, 1075, 1306, 1141, 448, 1315, 2274, 89, 586, 2123, 365, 515, 1639, 552, 1101, 285, 1209, 1770, 688, 2003, 1626, 693, 571, 1204, 1694, 700, 1065, 88, 92, 658, 1384, 788, 1631, 1828, 387, 2149, 1092, 775, 2153, 1516, 20, 1708, 1999, 2111, 1233, 574, 1242, 1548, 2214, 682, 1586, 2250, 346, 1254, 802, 27, 473, 766, 176, 2265, 824, 2019, 50, 1822, 1150, 1921, 905, 1265, 2172, 1248, 1334, 1898, 575, 1743, 2266, 1359, 429, 1925, 329, 2147, 2080, 1249, 399, 322, 660, 2032, 48, 299, 1696, 1742, 965, 595, 1767, 2050, 2049, 1087, 549, 1079, 12, 1942, 1260, 228, 770, 752, 337, 263, 1620, 2115, 1731, 716, 1615, 2219, 1863, 1227, 111, 403, 99, 557, 222, 257, 1330, 2231, 1422, 271, 2065, 1331, 1907, 625, 541, 491, 2098, 1704, 977, 1749, 2168, 806, 530, 1838, 1253, 922, 612, 2088, 256, 290, 8, 1567, 431, 1741, 1450, 731, 1328, 2113, 2180, 699, 1920, 613, 388, 624, 1727, 1244, 1484, 1352, 1963, 1536, 1428, 907, 1852, 1788, 1108, 1378, 946, 696, 1834, 1923, 281, 1263, 1433, 2086, 844, 2295, 580, 1755, 1471, 69, 1121, 871, 1655, 1667, 308, 2296, 1628, 634, 437, 1968, 1692, 537, 1425, 1528, 1595, 2253, 247, 983, 117, 1864, 39, 930, 895, 771, 1556, 1149, 1992, 1151, 1733, 1349, 306, 768, 1978, 560, 1559, 759, 638, 66, 210, 1459, 1927, 35, 454, 1948, 81, 1614, 1090, 1775, 578, 2158, 1624, 654, 1279, 416, 236, 2066, 2297, 2271, 181, 994, 2056, 1439, 1184, 1321, 1135, 1937, 157, 2236, 223, 1481, 1569, 1819, 2191, 349, 1813, 1404, 503, 1139, 189, 2237, 356, 1014, 120, 2222, 500, 1878, 377, 1499, 531, 948, 148, 1609, 1543, 1131, 1211, 718, 1473, 1668, 2193, 1032, 1635, 307, 1215, 1893, 2095, 230, 1375, 692, 185, 1796, 543, 1924, 1217, 720, 1272, 214, 1154, 422, 1544, 2260, 957, 511, 749, 996, 880, 1010, 1326, 1030, 49, 1820, 400, 1157, 1888, 1286, 2209, 603, 216, 30, 706, 928, 361, 61, 1868, 1525, 829, 438, 1590, 628, 1064, 1816, 747, 1690, 163, 1269, 213, 1511, 1722, 1147, 2093, 890, 1550, 138, 2218, 2157, 950, 1183, 1117, 1917, 277, 664, 679, 912, 2106, 1238, 2023, 1508, 725, 1376, 408, 2176, 533, 955, 672, 757, 205, 1673, 773, 577, 16, 1005, 1387, 487, 1309, 937, 1957, 1132, 1023, 746, 1268, 1170, 1402, 155, 529, 1642, 1039, 1939, 2061, 1145, 2107, 1155, 784, 2162, 1831, 1903, 1808, 2184, 2094, 2261, 932, 1351, 28, 1114, 1118, 1143, 761, 1250, 196, 1, 459, 1356, 540, 1995, 1654, 2279, 974, 787, 1003, 864, 1036, 2182, 1256, 1320, 274, 43, 2144, 627, 1672, 661, 1540, 158, 373, 1416, 1954, 1866, 1649, 173, 411, 2171, 737, 544, 2000, 1318, 743, 2028, 2226, 1527, 581, 2269, 690, 2277, 828, 554, 1110, 1084, 239, 215, 1273, 2244, 2276, 145, 392, 2154, 2138, 156, 1761, 90, 861, 1399, 842, 1322, 2283, 297, 1438, 434, 1236, 813, 1549, 1967, 255, 2285, 372, 587, 553, 460, 101, 1448, 945, 1583, 1562, 1875, 1986, 2035, 1581, 1989, 1052, 1946, 1830, 2006, 1395, 1054, 272, 1790, 1219, 703, 467, 1666, 1339, 1500, 2248, 1854, 293, 1440, 1941, 2151, 1522, 798, 1725, 1501, 1392, 1652, 840, 820, 468, 333, 1876, 1858, 2249, 336, 2206, 1518, 991, 1811, 762, 559, 60, 1344, 900, 1445, 1715, 998, 2067, 1177, 9, 2174, 1122, 380, 1613, 1420, 631, 1000, 733, 243, 1363, 1517, 283, 1134, 300, 1013, 1187, 229, 978, 1431, 1400, 2286, 289, 1009, 593, 2016, 1786, 990, 1086, 1140, 1019, 909, 1174, 324, 1538, 212, 1181, 1469, 923, 523, 1452, 827, 1323, 2110, 2105, 1207, 1826, 1231, 312, 348, 1373, 80, 426, 1301, 878, 1532, 1267, 199, 903, 1780, 789, 849, 995, 875, 1935, 602, 646, 2, 1494, 1784, 1243, 1936, 1633, 232, 527, 2262, 1098, 1403, 1338, 590, 1640, 1168, 1724, 1507, 852, 2268, 610, 635, 1610, 1601, 596, 671, 970, 993, 1966, 2102, 1847, 1290, 276, 474, 202, 327, 1902, 1208, 1444, 2012, 853, 1723, 147, 1228, 414, 1071, 268, 640, 1534, 370, 2064, 1357, 556, 1520, 1849, 1857, 744, 1564, 1240, 695, 507, 496, 1201, 1035, 1949, 1934, 1089, 2225, 1366, 286, 2227, 740, 1060, 2146, 1424, 465, 211, 854, 165, 1840, 177, 2298, 1504, 675, 837, 1095, 42, 1205, 484, 2070, 1612, 1845, 1997, 604, 2213, 2130, 1085, 1354, 513, 599, 107, 1524, 1669, 1397, 103, 2240, 490, 47, 1993, 252, 588, 1825, 742, 1276, 525, 763, 1837, 629, 894, 1580, 2005, 1618, 1589, 979, 777, 790, 104, 1519, 1824, 1498, 1247, 1568, 296, 1239, 1588, 608, 34, 729, 2220, 807, 294, 1222, 505, 2305, 1914, 674, 57, 618, 889, 1951, 84, 2200, 2030, 1879, 385, 1179, 973, 1915, 125, 1066, 1220, 2246, 485, 1271, 456, 1892, 676, 988, 471, 14, 1164, 1304, 1380, 1526, 1192, 404, 1884, 952, 893, 24, 2212, 162, 924, 959, 317, 65, 284, 1116, 1040, 573, 1765, 913, 539, 551, 2169, 1576, 811, 2134, 1191, 2114, 1463, 220, 1142, 1678, 1332, 2224, 986, 668, 1346, 154, 1983, 1717, 792, 1107, 1234, 1120, 2040, 1163, 940, 1103, 882, 262, 1407, 1510, 685, 245, 561, 1691, 72, 1284, 1028, 1778, 1546, 760, 949, 1465, 1367, 1461, 1630, 1533, 1955, 246, 1617, 1776, 2015, 1730, 1990, 1658, 1867, 680, 2062, 209, 961, 193, 1106, 1169, 1693, 1074, 819, 444, 1435, 1885, 1782, 269, 2223, 1597, 1213, 717, 1802, 1374, 2055, 2077, 2267, 1756, 1462, 2076, 748, 2104, 275, 447, 509, 1493, 1129, 1584, 1270, 1969, 2101, 1807, 1044, 2156, 248, 227, 1178, 519, 1821, 1607, 2198, 115, 797, 452, 342, 449, 1443, 200, 2047, 161, 1175, 2135, 328, 1769, 1472, 812, 1393, 266, 1909, 1734, 2234, 1970, 1947, 1159, 1193, 1700, 1477, 2264, 1726, 547, 77, 1091, 565, 1386, 1841, 1794, 704, 179, 755, 1991, 1076, 1919, 319, 354, 1246, 314, 38, 972, 427, 1833, 171, 1410, 1750, 457, 1045, 1758, 1800, 396, 194, 1611, 287, 956, 249, 776, 2073, 1912, 1280, 1952, 1335, 198, 2002, 2128, 398, 1100, 1008, 915, 1661, 1911, 705, 2211, 1046, 425, 1895, 121, 1274, 424, 470, 780, 1890, 2270, 1480, 570, 237, 238, 1940, 1283, 1763, 1647, 2241, 558, 36, 415, 195, 2121, 506, 1608, 2300, 52, 2299, 2278, 960, 1572, 954, 1412, 405, 1034, 1408, 1153, 169, 180, 620, 466, 1070, 1492, 1880, 536, 479, 622, 1396, 526, 1096, 665, 517, 2252, 2118, 673, 1119, 1636, 781, 2175, 791, 1771, 1883, 721, 476, 1012, 1069, 1038, 1729, 174, 2075, 662, 2204, 863, 178, 251, 918, 870, 152, 1732, 384, 1746, 2301, 1737, 340, 738, 1427, 393, 1853, 1237, 944, 1275, 891, 498, 1585, 428, 265, 1625, 1844, 1379, 188, 835, 391, 1575, 1762, 1757, 2045, 1460, 221, 1453, 598, 1928, 1189, 37, 1051, 1016, 1521, 1165, 2190, 1506, 2161, 785, 1973, 1541, 443, 2210, 1599, 1307, 2307, 1839, 1513, 1414, 2259, 1579, 217, 694, 1446, 2202, 1390, 2142, 1578, 848, 2155, 1906, 1047, 78, 2018, 614, 1785, 132, 1073, 1804, 881, 1146, 1020, 1972, 430, 1643, 1913, 1865, 850, 548, 2108, 857, 667, 1535, 159, 225, 841, 2021, 62, 1336, 330, 1112, 410, 822, 313, 2079
];


pub const NUM_WORDLE_VALID_WORDS: usize = 12971;

pub const WORDLE_VALID_WORDS: [WordleWord; NUM_WORDLE_VALID_WORDS] = [
	*b"AAHED",*b"AALII",*b"AARGH",*b"AARTI",*b"ABACA",*b"ABACI",*b"ABACK",*b"ABACS",*b"ABAFT",*b"ABAKA",
	*b"ABAMP",*b"ABAND",*b"ABASE",*b"ABASH",*b"ABASK",*b"ABATE",*b"ABAYA",*b"ABBAS",*b"ABBED",*b"ABBES",
	*b"ABBEY",*b"ABBOT",*b"ABCEE",*b"ABEAM",*b"ABEAR",*b"ABELE",*b"ABERS",*b"ABETS",*b"ABHOR",*b"ABIDE",
	*b"ABIES",*b"ABLED",*b"ABLER",*b"ABLES",*b"ABLET",*b"ABLOW",*b"ABMHO",*b"ABODE",*b"ABOHM",*b"ABOIL",
	*b"ABOMA",*b"ABOON",*b"ABORD",*b"ABORE",*b"ABORT",*b"ABOUT",*b"ABOVE",*b"ABRAM",*b"ABRAY",*b"ABRIM",
	*b"ABRIN",*b"ABRIS",*b"ABSEY",*b"ABSIT",*b"ABUNA",*b"ABUNE",*b"ABUSE",*b"ABUTS",*b"ABUZZ",*b"ABYES",
	*b"ABYSM",*b"ABYSS",*b"ACAIS",*b"ACARI",*b"ACCAS",*b"ACCOY",*b"ACERB",*b"ACERS",*b"ACETA",*b"ACHAR",
	*b"ACHED",*b"ACHES",*b"ACHOO",*b"ACIDS",*b"ACIDY",*b"ACING",*b"ACINI",*b"ACKEE",*b"ACKER",*b"ACMES",
	*b"ACMIC",*b"ACNED",*b"ACNES",*b"ACOCK",*b"ACOLD",*b"ACORN",*b"ACRED",*b"ACRES",*b"ACRID",*b"ACROS",
	*b"ACTED",*b"ACTIN",*b"ACTON",*b"ACTOR",*b"ACUTE",*b"ACYLS",*b"ADAGE",*b"ADAPT",*b"ADAWS",*b"ADAYS",
	*b"ADBOT",*b"ADDAX",*b"ADDED",*b"ADDER",*b"ADDIO",*b"ADDLE",*b"ADEEM",*b"ADEPT",*b"ADHAN",*b"ADIEU",
	*b"ADIOS",*b"ADITS",*b"ADMAN",*b"ADMEN",*b"ADMIN",*b"ADMIT",*b"ADMIX",*b"ADOBE",*b"ADOBO",*b"ADOPT",
	*b"ADORE",*b"ADORN",*b"ADOWN",*b"ADOZE",*b"ADRAD",*b"ADRED",*b"ADSUM",*b"ADUKI",*b"ADULT",*b"ADUNC",
	*b"ADUST",*b"ADVEW",*b"ADYTA",*b"ADZED",*b"ADZES",*b"AECIA",*b"AEDES",*b"AEGIS",*b"AEONS",*b"AERIE",
	*b"AEROS",*b"AESIR",*b"AFALD",*b"AFARA",*b"AFARS",*b"AFEAR",*b"AFFIX",*b"AFIRE",*b"AFLAJ",*b"AFOOT",
	*b"AFORE",*b"AFOUL",*b"AFRIT",*b"AFROS",*b"AFTER",*b"AGAIN",*b"AGAMA",*b"AGAMI",*b"AGAPE",*b"AGARS",
	*b"AGAST",*b"AGATE",*b"AGAVE",*b"AGAZE",*b"AGENE",*b"AGENT",*b"AGERS",*b"AGGER",*b"AGGIE",*b"AGGRI",
	*b"AGGRO",*b"AGGRY",*b"AGHAS",*b"AGILA",*b"AGILE",*b"AGING",*b"AGIOS",*b"AGISM",*b"AGIST",*b"AGITA",
	*b"AGLEE",*b"AGLET",*b"AGLEY",*b"AGLOO",*b"AGLOW",*b"AGLUS",*b"AGMAS",*b"AGOGE",*b"AGONE",*b"AGONS",
	*b"AGONY",*b"AGOOD",*b"AGORA",*b"AGREE",*b"AGRIA",*b"AGRIN",*b"AGROS",*b"AGUED",*b"AGUES",*b"AGUNA",
	*b"AGUTI",*b"AHEAD",*b"AHEAP",*b"AHENT",*b"AHIGH",*b"AHIND",*b"AHING",*b"AHINT",*b"AHOLD",*b"AHULL",
	*b"AHURU",*b"AIDAS",*b"AIDED",*b"AIDER",*b"AIDES",*b"AIDOI",*b"AIDOS",*b"AIERY",*b"AIGAS",*b"AIGHT",
	*b"AILED",*b"AIMED",*b"AIMER",*b"AINEE",*b"AINGA",*b"AIOLI",*b"AIRED",*b"AIRER",*b"AIRNS",*b"AIRTH",
	*b"AIRTS",*b"AISLE",*b"AITCH",*b"AITUS",*b"AIVER",*b"AIYEE",*b"AIZLE",*b"AJIES",*b"AJIVA",*b"AJUGA",
	*b"AJWAN",*b"AKEES",*b"AKELA",*b"AKENE",*b"AKING",*b"AKITA",*b"AKKAS",*b"ALAAP",*b"ALACK",*b"ALAMO",
	*b"ALAND",*b"ALANE",*b"ALANG",*b"ALANS",*b"ALANT",*b"ALAPA",*b"ALAPS",*b"ALARM",*b"ALARY",*b"ALATE",
	*b"ALAYS",*b"ALBAS",*b"ALBEE",*b"ALBUM",*b"ALCID",*b"ALCOS",*b"ALDEA",*b"ALDER",*b"ALDOL",*b"ALECK",
	*b"ALECS",*b"ALEFS",*b"ALEFT",*b"ALEPH",*b"ALERT",*b"ALEWS",*b"ALEYE",*b"ALFAS",*b"ALGAE",*b"ALGAL",
	*b"ALGAS",*b"ALGID",*b"ALGIN",*b"ALGOR",*b"ALGUM",*b"ALIAS",*b"ALIBI",*b"ALIEN",*b"ALIFS",*b"ALIGN",
	*b"ALIKE",*b"ALINE",*b"ALIST",*b"ALIVE",*b"ALIYA",*b"ALKIE",*b"ALKOS",*b"ALKYD",*b"ALKYL",*b"ALLAY",
	*b"ALLEE",*b"ALLEL",*b"ALLEY",*b"ALLIS",*b"ALLOD",*b"ALLOT",*b"ALLOW",*b"ALLOY",*b"ALLYL",*b"ALMAH",
	*b"ALMAS",*b"ALMEH",*b"ALMES",*b"ALMUD",*b"ALMUG",*b"ALODS",*b"ALOED",*b"ALOES",*b"ALOFT",*b"ALOHA",
	*b"ALOIN",*b"ALONE",*b"ALONG",*b"ALOOF",*b"ALOOS",*b"ALOUD",*b"ALOWE",*b"ALPHA",*b"ALTAR",*b"ALTER",
	*b"ALTHO",*b"ALTOS",*b"ALULA",*b"ALUMS",*b"ALURE",*b"ALVAR",*b"ALWAY",*b"AMAHS",*b"AMAIN",*b"AMASS",
	*b"AMATE",*b"AMAUT",*b"AMAZE",*b"AMBAN",*b"AMBER",*b"AMBIT",*b"AMBLE",*b"AMBOS",*b"AMBRY",*b"AMEBA",
	*b"AMEER",*b"AMEND",*b"AMENE",*b"AMENS",*b"AMENT",*b"AMIAS",*b"AMICE",*b"AMICI",*b"AMIDE",*b"AMIDO",
	*b"AMIDS",*b"AMIES",*b"AMIGA",*b"AMIGO",*b"AMINE",*b"AMINO",*b"AMINS",*b"AMIRS",*b"AMISS",*b"AMITY",
	*b"AMLAS",*b"AMMAN",*b"AMMON",*b"AMMOS",*b"AMNIA",*b"AMNIC",*b"AMNIO",*b"AMOKS",*b"AMOLE",*b"AMONG",
	*b"AMORT",*b"AMOUR",*b"AMOVE",*b"AMOWT",*b"AMPED",*b"AMPLE",*b"AMPLY",*b"AMPUL",*b"AMRIT",*b"AMUCK",
	*b"AMUSE",*b"AMYLS",*b"ANANA",*b"ANATA",*b"ANCHO",*b"ANCLE",*b"ANCON",*b"ANDRO",*b"ANEAR",*b"ANELE",
	*b"ANENT",*b"ANGAS",*b"ANGEL",*b"ANGER",*b"ANGLE",*b"ANGLO",*b"ANGRY",*b"ANGST",*b"ANIGH",*b"ANILE",
	*b"ANILS",*b"ANIMA",*b"ANIME",*b"ANIMI",*b"ANION",*b"ANISE",*b"ANKER",*b"ANKHS",*b"ANKLE",*b"ANKUS",
	*b"ANLAS",*b"ANNAL",*b"ANNAS",*b"ANNAT",*b"ANNEX",*b"ANNOY",*b"ANNUL",*b"ANOAS",*b"ANODE",*b"ANOLE",
	*b"ANOMY",*b"ANSAE",*b"ANTAE",*b"ANTAR",*b"ANTAS",*b"ANTED",*b"ANTES",*b"ANTIC",*b"ANTIS",*b"ANTRA",
	*b"ANTRE",*b"ANTSY",*b"ANURA",*b"ANVIL",*b"ANYON",*b"AORTA",*b"APACE",*b"APAGE",*b"APAID",*b"APART",
	*b"APAYD",*b"APAYS",*b"APEAK",*b"APEEK",*b"APERS",*b"APERT",*b"APERY",*b"APGAR",*b"APHID",*b"APHIS",
	*b"APIAN",*b"APING",*b"APIOL",*b"APISH",*b"APISM",*b"APNEA",*b"APODE",*b"APODS",*b"APOOP",*b"APORT",
	*b"APPAL",*b"APPAY",*b"APPEL",*b"APPLE",*b"APPLY",*b"APPRO",*b"APPUI",*b"APPUY",*b"APRES",*b"APRON",
	*b"APSES",*b"APSIS",*b"APSOS",*b"APTED",*b"APTER",*b"APTLY",*b"AQUAE",*b"AQUAS",*b"ARABA",*b"ARAKS",
	*b"ARAME",*b"ARARS",*b"ARBAS",*b"ARBOR",*b"ARCED",*b"ARCHI",*b"ARCOS",*b"ARCUS",*b"ARDEB",*b"ARDOR",
	*b"ARDRI",*b"AREAD",*b"AREAE",*b"AREAL",*b"AREAR",*b"AREAS",*b"ARECA",*b"AREDD",*b"AREDE",*b"AREFY",
	*b"AREIC",*b"ARENA",*b"ARENE",*b"AREPA",*b"ARERE",*b"ARETE",*b"ARETS",*b"ARETT",*b"ARGAL",*b"ARGAN",
	*b"ARGIL",*b"ARGLE",*b"ARGOL",*b"ARGON",*b"ARGOT",*b"ARGUE",*b"ARGUS",*b"ARHAT",*b"ARIAS",*b"ARIEL",
	*b"ARIKI",*b"ARILS",*b"ARIOT",*b"ARISE",*b"ARISH",*b"ARKED",*b"ARLED",*b"ARLES",*b"ARMED",*b"ARMER",
	*b"ARMET",*b"ARMIL",*b"ARMOR",*b"ARNAS",*b"ARNUT",*b"AROBA",*b"AROHA",*b"AROID",*b"AROMA",*b"AROSE",
	*b"ARPAS",*b"ARPEN",*b"ARRAH",*b"ARRAS",*b"ARRAY",*b"ARRET",*b"ARRIS",*b"ARROW",*b"ARROZ",*b"ARSED",
	*b"ARSES",*b"ARSEY",*b"ARSIS",*b"ARSON",*b"ARTAL",*b"ARTEL",*b"ARTIC",*b"ARTIS",*b"ARTSY",*b"ARUHE",
	*b"ARUMS",*b"ARVAL",*b"ARVEE",*b"ARVOS",*b"ARYLS",*b"ASANA",*b"ASCON",*b"ASCOT",*b"ASCUS",*b"ASDIC",
	*b"ASHED",*b"ASHEN",*b"ASHES",*b"ASHET",*b"ASIDE",*b"ASKED",*b"ASKER",*b"ASKEW",*b"ASKOI",*b"ASKOS",
	*b"ASPEN",*b"ASPER",*b"ASPIC",*b"ASPIE",*b"ASPIS",*b"ASPRO",*b"ASSAI",*b"ASSAM",*b"ASSAY",*b"ASSES",
	*b"ASSET",*b"ASSEZ",*b"ASSOT",*b"ASTER",*b"ASTIR",*b"ASTUN",*b"ASURA",*b"ASWAY",*b"ASWIM",*b"ASYLA",
	*b"ATAPS",*b"ATAXY",*b"ATIGI",*b"ATILT",*b"ATIMY",*b"ATLAS",*b"ATMAN",*b"ATMAS",*b"ATMOS",*b"ATOCS",
	*b"ATOKE",*b"ATOKS",*b"ATOLL",*b"ATOMS",*b"ATOMY",*b"ATONE",*b"ATONY",*b"ATOPY",*b"ATRIA",*b"ATRIP",
	*b"ATTAP",*b"ATTAR",*b"ATTIC",*b"ATUAS",*b"AUDAD",*b"AUDIO",*b"AUDIT",*b"AUGER",*b"AUGHT",*b"AUGUR",
	*b"AULAS",*b"AULIC",*b"AULOI",*b"AULOS",*b"AUMIL",*b"AUNES",*b"AUNTS",*b"AUNTY",*b"AURAE",*b"AURAL",
	*b"AURAR",*b"AURAS",*b"AUREI",*b"AURES",*b"AURIC",*b"AURIS",*b"AURUM",*b"AUTOS",*b"AUXIN",*b"AVAIL",
	*b"AVALE",*b"AVANT",*b"AVAST",*b"AVELS",*b"AVENS",*b"AVERS",*b"AVERT",*b"AVGAS",*b"AVIAN",*b"AVINE",
	*b"AVION",*b"AVISE",*b"AVISO",*b"AVIZE",*b"AVOID",*b"AVOWS",*b"AVYZE",*b"AWAIT",*b"AWAKE",*b"AWARD",
	*b"AWARE",*b"AWARN",*b"AWASH",*b"AWATO",*b"AWAVE",*b"AWAYS",*b"AWDLS",*b"AWEEL",*b"AWETO",*b"AWFUL",
	*b"AWING",*b"AWMRY",*b"AWNED",*b"AWNER",*b"AWOKE",*b"AWOLS",*b"AWORK",*b"AXELS",*b"AXIAL",*b"AXILE",
	*b"AXILS",*b"AXING",*b"AXIOM",*b"AXION",*b"AXITE",*b"AXLED",*b"AXLES",*b"AXMAN",*b"AXMEN",*b"AXOID",
	*b"AXONE",*b"AXONS",*b"AYAHS",*b"AYAYA",*b"AYELP",*b"AYGRE",*b"AYINS",*b"AYONT",*b"AYRES",*b"AYRIE",
	*b"AZANS",*b"AZIDE",*b"AZIDO",*b"AZINE",*b"AZLON",*b"AZOIC",*b"AZOLE",*b"AZONS",*b"AZOTE",*b"AZOTH",
	*b"AZUKI",*b"AZURE",*b"AZURN",*b"AZURY",*b"AZYGY",*b"AZYME",*b"AZYMS",*b"BAAED",*b"BAALS",*b"BABAS",
	*b"BABEL",*b"BABES",*b"BABKA",*b"BABOO",*b"BABUL",*b"BABUS",*b"BACCA",*b"BACCO",*b"BACCY",*b"BACHA",
	*b"BACHS",*b"BACKS",*b"BACON",*b"BADDY",*b"BADGE",*b"BADLY",*b"BAELS",*b"BAFFS",*b"BAFFY",*b"BAFTS",
	*b"BAGEL",*b"BAGGY",*b"BAGHS",*b"BAGIE",*b"BAHTS",*b"BAHUS",*b"BAHUT",*b"BAILS",*b"BAIRN",*b"BAISA",
	*b"BAITH",*b"BAITS",*b"BAIZA",*b"BAIZE",*b"BAJAN",*b"BAJRA",*b"BAJRI",*b"BAJUS",*b"BAKED",*b"BAKEN",
	*b"BAKER",*b"BAKES",*b"BAKRA",*b"BALAS",*b"BALDS",*b"BALDY",*b"BALED",*b"BALER",*b"BALES",*b"BALKS",
	*b"BALKY",*b"BALLS",*b"BALLY",*b"BALMS",*b"BALMY",*b"BALOO",*b"BALSA",*b"BALTI",*b"BALUN",*b"BALUS",
	*b"BAMBI",*b"BANAK",*b"BANAL",*b"BANCO",*b"BANCS",*b"BANDA",*b"BANDH",*b"BANDS",*b"BANDY",*b"BANED",
	*b"BANES",*b"BANGS",*b"BANIA",*b"BANJO",*b"BANKS",*b"BANNS",*b"BANTS",*b"BANTU",*b"BANTY",*b"BANYA",
	*b"BAPUS",*b"BARBE",*b"BARBS",*b"BARBY",*b"BARCA",*b"BARDE",*b"BARDO",*b"BARDS",*b"BARDY",*b"BARED",
	*b"BARER",*b"BARES",*b"BARFI",*b"BARFS",*b"BARGE",*b"BARIC",*b"BARKS",*b"BARKY",*b"BARMS",*b"BARMY",
	*b"BARNS",*b"BARNY",*b"BARON",*b"BARPS",*b"BARRA",*b"BARRE",*b"BARRO",*b"BARRY",*b"BARYE",*b"BASAL",
	*b"BASAN",*b"BASED",*b"BASEN",*b"BASER",*b"BASES",*b"BASHO",*b"BASIC",*b"BASIJ",*b"BASIL",*b"BASIN",
	*b"BASIS",*b"BASKS",*b"BASON",*b"BASSE",*b"BASSI",*b"BASSO",*b"BASSY",*b"BASTA",*b"BASTE",*b"BASTI",
	*b"BASTO",*b"BASTS",*b"BATCH",*b"BATED",*b"BATES",*b"BATHE",*b"BATHS",*b"BATIK",*b"BATON",*b"BATTA",
	*b"BATTS",*b"BATTU",*b"BATTY",*b"BAUDS",*b"BAUKS",*b"BAULK",*b"BAURS",*b"BAVIN",*b"BAWDS",*b"BAWDY",
	*b"BAWKS",*b"BAWLS",*b"BAWNS",*b"BAWRS",*b"BAWTY",*b"BAYED",*b"BAYER",*b"BAYES",*b"BAYLE",*b"BAYOU",
	*b"BAYTS",*b"BAZAR",*b"BAZOO",*b"BEACH",*b"BEADS",*b"BEADY",*b"BEAKS",*b"BEAKY",*b"BEALS",*b"BEAMS",
	*b"BEAMY",*b"BEANO",*b"BEANS",*b"BEANY",*b"BEARD",*b"BEARE",*b"BEARS",*b"BEAST",*b"BEATH",*b"BEATS",
	*b"BEATY",*b"BEAUS",*b"BEAUT",*b"BEAUX",*b"BEBOP",*b"BECAP",*b"BECKE",*b"BECKS",*b"BEDAD",*b"BEDEL",
	*b"BEDES",*b"BEDEW",*b"BEDIM",*b"BEDYE",*b"BEECH",*b"BEEDI",*b"BEEFS",*b"BEEFY",*b"BEEPS",*b"BEERS",
	*b"BEERY",*b"BEETS",*b"BEFIT",*b"BEFOG",*b"BEGAD",*b"BEGAN",*b"BEGAR",*b"BEGAT",*b"BEGEM",*b"BEGET",
	*b"BEGIN",*b"BEGOT",*b"BEGUM",*b"BEGUN",*b"BEIGE",*b"BEIGY",*b"BEING",*b"BEINS",*b"BEKAH",*b"BELAH",
	*b"BELAR",*b"BELAY",*b"BELCH",*b"BELEE",*b"BELGA",*b"BELIE",*b"BELLE",*b"BELLS",*b"BELLY",*b"BELON",
	*b"BELOW",*b"BELTS",*b"BEMAD",*b"BEMAS",*b"BEMIX",*b"BEMUD",*b"BENCH",*b"BENDS",*b"BENDY",*b"BENES",
	*b"BENET",*b"BENGA",*b"BENIS",*b"BENNE",*b"BENNI",*b"BENNY",*b"BENTO",*b"BENTS",*b"BENTY",*b"BEPAT",
	*b"BERAY",*b"BERES",*b"BERET",*b"BERGS",*b"BERKO",*b"BERKS",*b"BERME",*b"BERMS",*b"BEROB",*b"BERRY",
	*b"BERTH",*b"BERYL",*b"BESAT",*b"BESAW",*b"BESEE",*b"BESES",*b"BESET",*b"BESIT",*b"BESOM",*b"BESOT",
	*b"BESTI",*b"BESTS",*b"BETAS",*b"BETED",*b"BETEL",*b"BETES",*b"BETHS",*b"BETID",*b"BETON",*b"BETTA",
	*b"BETTY",*b"BEVEL",*b"BEVER",*b"BEVOR",*b"BEVUE",*b"BEVVY",*b"BEWET",*b"BEWIG",*b"BEZEL",*b"BEZES",
	*b"BEZIL",*b"BEZZY",*b"BHAIS",*b"BHAJI",*b"BHANG",*b"BHATS",*b"BHELS",*b"BHOOT",*b"BHUNA",*b"BHUTS",
	*b"BIACH",*b"BIALI",*b"BIALY",*b"BIBBS",*b"BIBES",*b"BIBLE",*b"BICCY",*b"BICEP",*b"BICES",*b"BIDDY",
	*b"BIDED",*b"BIDER",*b"BIDES",*b"BIDET",*b"BIDIS",*b"BIDON",*b"BIELD",*b"BIERS",*b"BIFFO",*b"BIFFS",
	*b"BIFFY",*b"BIFID",*b"BIGAE",*b"BIGGS",*b"BIGGY",*b"BIGHA",*b"BIGHT",*b"BIGLY",*b"BIGOS",*b"BIGOT",
	*b"BIJOU",*b"BIKED",*b"BIKER",*b"BIKES",*b"BIKIE",*b"BILBO",*b"BILBY",*b"BILED",*b"BILES",*b"BILGE",
	*b"BILGY",*b"BILKS",*b"BILLS",*b"BILLY",*b"BIMAH",*b"BIMAS",*b"BIMBO",*b"BINAL",*b"BINDI",*b"BINDS",
	*b"BINER",*b"BINES",*b"BINGE",*b"BINGO",*b"BINGS",*b"BINGY",*b"BINIT",*b"BINKS",*b"BINTS",*b"BIOGS",
	*b"BIOME",*b"BIONT",*b"BIOTA",*b"BIPED",*b"BIPOD",*b"BIRCH",*b"BIRDS",*b"BIRKS",*b"BIRLE",*b"BIRLS",
	*b"BIROS",*b"BIRRS",*b"BIRSE",*b"BIRSY",*b"BIRTH",*b"BISES",*b"BISKS",*b"BISOM",*b"BISON",*b"BITCH",
	*b"BITER",*b"BITES",*b"BITOS",*b"BITOU",*b"BITSY",*b"BITTE",*b"BITTS",*b"BITTY",*b"BIVIA",*b"BIVVY",
	*b"BIZES",*b"BIZZO",*b"BIZZY",*b"BLABS",*b"BLACK",*b"BLADE",*b"BLADS",*b"BLADY",*b"BLAER",*b"BLAES",
	*b"BLAFF",*b"BLAGS",*b"BLAHS",*b"BLAIN",*b"BLAME",*b"BLAMS",*b"BLAND",*b"BLANK",*b"BLARE",*b"BLART",
	*b"BLASE",*b"BLASH",*b"BLAST",*b"BLATE",*b"BLATS",*b"BLATT",*b"BLAUD",*b"BLAWN",*b"BLAWS",*b"BLAYS",
	*b"BLAZE",*b"BLEAK",*b"BLEAR",*b"BLEAT",*b"BLEBS",*b"BLECH",*b"BLEED",*b"BLEEP",*b"BLEES",*b"BLEND",
	*b"BLENT",*b"BLERT",*b"BLESS",*b"BLEST",*b"BLETS",*b"BLEYS",*b"BLIMP",*b"BLIMY",*b"BLIND",*b"BLING",
	*b"BLINI",*b"BLINK",*b"BLINS",*b"BLINY",*b"BLIPS",*b"BLISS",*b"BLIST",*b"BLITE",*b"BLITS",*b"BLITZ",
	*b"BLIVE",*b"BLOAT",*b"BLOBS",*b"BLOCK",*b"BLOCS",*b"BLOGS",*b"BLOKE",*b"BLOND",*b"BLOOD",*b"BLOOK",
	*b"BLOOM",*b"BLOOP",*b"BLORE",*b"BLOTS",*b"BLOWN",*b"BLOWS",*b"BLOWY",*b"BLUBS",*b"BLUDE",*b"BLUDS",
	*b"BLUDY",*b"BLUED",*b"BLUER",*b"BLUES",*b"BLUET",*b"BLUEY",*b"BLUFF",*b"BLUID",*b"BLUME",*b"BLUNK",
	*b"BLUNT",*b"BLURB",*b"BLURS",*b"BLURT",*b"BLUSH",*b"BLYPE",*b"BOABS",*b"BOAKS",*b"BOARD",*b"BOARS",
	*b"BOART",*b"BOAST",*b"BOATS",*b"BOBAC",*b"BOBAK",*b"BOBAS",*b"BOBBY",*b"BOBOL",*b"BOBOS",*b"BOCCA",
	*b"BOCCE",*b"BOCCI",*b"BOCHE",*b"BOCKS",*b"BODED",*b"BODES",*b"BODGE",*b"BODHI",*b"BODLE",*b"BOEPS",
	*b"BOETS",*b"BOEUF",*b"BOFFO",*b"BOFFS",*b"BOGAN",*b"BOGEY",*b"BOGGY",*b"BOGIE",*b"BOGLE",*b"BOGUE",
	*b"BOGUS",*b"BOHEA",*b"BOHOS",*b"BOILS",*b"BOING",*b"BOINK",*b"BOITE",*b"BOKED",*b"BOKEH",*b"BOKES",
	*b"BOKOS",*b"BOLAR",*b"BOLAS",*b"BOLDS",*b"BOLES",*b"BOLIX",*b"BOLLS",*b"BOLOS",*b"BOLTS",*b"BOLUS",
	*b"BOMAS",*b"BOMBE",*b"BOMBO",*b"BOMBS",*b"BONCE",*b"BONDS",*b"BONED",*b"BONER",*b"BONES",*b"BONEY",
	*b"BONGO",*b"BONGS",*b"BONIE",*b"BONKS",*b"BONNE",*b"BONNY",*b"BONUS",*b"BONZA",*b"BONZE",*b"BOOAI",
	*b"BOOAY",*b"BOOBS",*b"BOOBY",*b"BOODY",*b"BOOED",*b"BOOFY",*b"BOOGY",*b"BOOHS",*b"BOOKS",*b"BOOKY",
	*b"BOOLS",*b"BOOMS",*b"BOOMY",*b"BOONG",*b"BOONS",*b"BOORD",*b"BOORS",*b"BOOSE",*b"BOOST",*b"BOOTH",
	*b"BOOTS",*b"BOOTY",*b"BOOZE",*b"BOOZY",*b"BOPPY",*b"BORAK",*b"BORAL",*b"BORAS",*b"BORAX",*b"BORDE",
	*b"BORDS",*b"BORED",*b"BOREE",*b"BOREL",*b"BORER",*b"BORES",*b"BORGO",*b"BORIC",*b"BORKS",*b"BORMS",
	*b"BORNA",*b"BORNE",*b"BORON",*b"BORTS",*b"BORTY",*b"BORTZ",*b"BOSIE",*b"BOSKS",*b"BOSKY",*b"BOSOM",
	*b"BOSON",*b"BOSSY",*b"BOSUN",*b"BOTAS",*b"BOTCH",*b"BOTEL",*b"BOTES",*b"BOTHY",*b"BOTTE",*b"BOTTS",
	*b"BOTTY",*b"BOUGE",*b"BOUGH",*b"BOUKS",*b"BOULE",*b"BOULT",*b"BOUND",*b"BOUNS",*b"BOURD",*b"BOURG",
	*b"BOURN",*b"BOUSE",*b"BOUSY",*b"BOUTS",*b"BOVID",*b"BOWAT",*b"BOWED",*b"BOWEL",*b"BOWER",*b"BOWES",
	*b"BOWET",*b"BOWIE",*b"BOWLS",*b"BOWNE",*b"BOWRS",*b"BOWSE",*b"BOXED",*b"BOXEN",*b"BOXER",*b"BOXES",
	*b"BOXLA",*b"BOXTY",*b"BOYAR",*b"BOYAU",*b"BOYED",*b"BOYFS",*b"BOYGS",*b"BOYLA",*b"BOYOS",*b"BOYSY",
	*b"BOZOS",*b"BRAAI",*b"BRACE",*b"BRACH",*b"BRACK",*b"BRACT",*b"BRADS",*b"BRAES",*b"BRAGS",*b"BRAID",
	*b"BRAIL",*b"BRAIN",*b"BRAKE",*b"BRAKS",*b"BRAKY",*b"BRAME",*b"BRAND",*b"BRANE",*b"BRANK",*b"BRANS",
	*b"BRANT",*b"BRASH",*b"BRASS",*b"BRAST",*b"BRATS",*b"BRAVA",*b"BRAVE",*b"BRAVI",*b"BRAVO",*b"BRAWL",
	*b"BRAWN",*b"BRAWS",*b"BRAXY",*b"BRAYS",*b"BRAZA",*b"BRAZE",*b"BREAD",*b"BREAK",*b"BREAM",*b"BREDE",
	*b"BREDS",*b"BREED",*b"BREEM",*b"BREER",*b"BREES",*b"BREID",*b"BREIS",*b"BREME",*b"BRENS",*b"BRENT",
	*b"BRERE",*b"BRERS",*b"BREVE",*b"BREWS",*b"BREYS",*b"BRIAR",*b"BRIBE",*b"BRICK",*b"BRIDE",*b"BRIEF",
	*b"BRIER",*b"BRIES",*b"BRIGS",*b"BRIKI",*b"BRIKS",*b"BRILL",*b"BRIMS",*b"BRINE",*b"BRING",*b"BRINK",
	*b"BRINS",*b"BRINY",*b"BRIOS",*b"BRISE",*b"BRISK",*b"BRISS",*b"BRITH",*b"BRITS",*b"BRITT",*b"BRIZE",
	*b"BROAD",*b"BROCH",*b"BROCK",*b"BRODS",*b"BROGH",*b"BROGS",*b"BROIL",*b"BROKE",*b"BROME",*b"BROMO",
	*b"BRONC",*b"BROND",*b"BROOD",*b"BROOK",*b"BROOL",*b"BROOM",*b"BROOS",*b"BROSE",*b"BROSY",*b"BROTH",
	*b"BROWN",*b"BROWS",*b"BRUGH",*b"BRUIN",*b"BRUIT",*b"BRULE",*b"BRUME",*b"BRUNG",*b"BRUNT",*b"BRUSH",
	*b"BRUSK",*b"BRUST",*b"BRUTE",*b"BRUTS",*b"BUATS",*b"BUAZE",*b"BUBAL",*b"BUBAS",*b"BUBBA",*b"BUBBE",
	*b"BUBBY",*b"BUBUS",*b"BUCHU",*b"BUCKO",*b"BUCKS",*b"BUCKU",*b"BUDAS",*b"BUDDY",*b"BUDGE",*b"BUDIS",
	*b"BUDOS",*b"BUFFA",*b"BUFFE",*b"BUFFI",*b"BUFFO",*b"BUFFS",*b"BUFFY",*b"BUFOS",*b"BUFTY",*b"BUGGY",
	*b"BUGLE",*b"BUHLS",*b"BUHRS",*b"BUIKS",*b"BUILD",*b"BUILT",*b"BUIST",*b"BUKES",*b"BULBS",*b"BULGE",
	*b"BULGY",*b"BULKS",*b"BULKY",*b"BULLA",*b"BULLS",*b"BULLY",*b"BULSE",*b"BUMBO",*b"BUMFS",*b"BUMPH",
	*b"BUMPS",*b"BUMPY",*b"BUNAS",*b"BUNCE",*b"BUNCH",*b"BUNCO",*b"BUNDE",*b"BUNDH",*b"BUNDS",*b"BUNDT",
	*b"BUNDU",*b"BUNDY",*b"BUNGS",*b"BUNGY",*b"BUNIA",*b"BUNJE",*b"BUNJY",*b"BUNKO",*b"BUNKS",*b"BUNNS",
	*b"BUNNY",*b"BUNTS",*b"BUNTY",*b"BUNYA",*b"BUOYS",*b"BUPPY",*b"BURAN",*b"BURAS",*b"BURBS",*b"BURDS",
	*b"BURET",*b"BURFI",*b"BURGH",*b"BURGS",*b"BURIN",*b"BURKA",*b"BURKE",*b"BURKS",*b"BURLS",*b"BURLY",
	*b"BURNS",*b"BURNT",*b"BUROO",*b"BURPS",*b"BURQA",*b"BURRO",*b"BURRS",*b"BURRY",*b"BURSA",*b"BURSE",
	*b"BURST",*b"BUSBY",*b"BUSED",*b"BUSES",*b"BUSHY",*b"BUSKS",*b"BUSKY",*b"BUSSU",*b"BUSTI",*b"BUSTS",
	*b"BUSTY",*b"BUTCH",*b"BUTEO",*b"BUTES",*b"BUTLE",*b"BUTOH",*b"BUTTE",*b"BUTTS",*b"BUTTY",*b"BUTUT",
	*b"BUTYL",*b"BUXOM",*b"BUYER",*b"BUZZY",*b"BWANA",*b"BWAZI",*b"BYDED",*b"BYDES",*b"BYKED",*b"BYKES",
	*b"BYLAW",*b"BYRES",*b"BYRLS",*b"BYSSI",*b"BYTES",*b"BYWAY",*b"CAAED",*b"CABAL",*b"CABAS",*b"CABBY",
	*b"CABER",*b"CABIN",*b"CABLE",*b"CABOB",*b"CABOC",*b"CABRE",*b"CACAO",*b"CACAS",*b"CACHE",*b"CACKS",
	*b"CACKY",*b"CACTI",*b"CADDY",*b"CADEE",*b"CADES",*b"CADET",*b"CADGE",*b"CADGY",*b"CADIE",*b"CADIS",
	*b"CADRE",*b"CAECA",*b"CAESE",*b"CAFES",*b"CAFFS",*b"CAGED",*b"CAGER",*b"CAGES",*b"CAGEY",*b"CAGOT",
	*b"CAHOW",*b"CAIDS",*b"CAINS",*b"CAIRD",*b"CAIRN",*b"CAJON",*b"CAJUN",*b"CAKED",*b"CAKES",*b"CAKEY",
	*b"CALFS",*b"CALID",*b"CALIF",*b"CALIX",*b"CALKS",*b"CALLA",*b"CALLS",*b"CALMS",*b"CALMY",*b"CALOS",
	*b"CALPA",*b"CALPS",*b"CALVE",*b"CALYX",*b"CAMAN",*b"CAMAS",*b"CAMEL",*b"CAMEO",*b"CAMES",*b"CAMIS",
	*b"CAMOS",*b"CAMPI",*b"CAMPO",*b"CAMPS",*b"CAMPY",*b"CAMUS",*b"CANAL",*b"CANDY",*b"CANED",*b"CANEH",
	*b"CANER",*b"CANES",*b"CANGS",*b"CANID",*b"CANNA",*b"CANNS",*b"CANNY",*b"CANOE",*b"CANON",*b"CANSO",
	*b"CANST",*b"CANTO",*b"CANTS",*b"CANTY",*b"CAPAS",*b"CAPED",*b"CAPER",*b"CAPES",*b"CAPEX",*b"CAPHS",
	*b"CAPIZ",*b"CAPLE",*b"CAPON",*b"CAPOS",*b"CAPOT",*b"CAPRI",*b"CAPUL",*b"CAPUT",*b"CARAP",*b"CARAT",
	*b"CARBO",*b"CARBS",*b"CARBY",*b"CARDI",*b"CARDS",*b"CARDY",*b"CARED",*b"CARER",*b"CARES",*b"CARET",
	*b"CAREX",*b"CARGO",*b"CARKS",*b"CARLE",*b"CARLS",*b"CARNS",*b"CARNY",*b"CAROB",*b"CAROL",*b"CAROM",
	*b"CARON",*b"CARPI",*b"CARPS",*b"CARRS",*b"CARRY",*b"CARSE",*b"CARTA",*b"CARTE",*b"CARTS",*b"CARVE",
	*b"CARVY",*b"CASAS",*b"CASCO",*b"CASED",*b"CASES",*b"CASKS",*b"CASKY",*b"CASTE",*b"CASTS",*b"CASUS",
	*b"CATCH",*b"CATER",*b"CATES",*b"CATTY",*b"CAUDA",*b"CAUKS",*b"CAULD",*b"CAULK",*b"CAULS",*b"CAUMS",
	*b"CAUPS",*b"CAURI",*b"CAUSA",*b"CAUSE",*b"CAVAS",*b"CAVED",*b"CAVEL",*b"CAVER",*b"CAVES",*b"CAVIE",
	*b"CAVIL",*b"CAWED",*b"CAWKS",*b"CAXON",*b"CEASE",*b"CEAZE",*b"CEBID",*b"CECAL",*b"CECUM",*b"CEDAR",
	*b"CEDED",*b"CEDER",*b"CEDES",*b"CEDIS",*b"CEIBA",*b"CEILI",*b"CEILS",*b"CELEB",*b"CELLA",*b"CELLI",
	*b"CELLO",*b"CELLS",*b"CELOM",*b"CELTS",*b"CENSE",*b"CENTO",*b"CENTS",*b"CENTU",*b"CEORL",*b"CEPES",
	*b"CERCI",*b"CERED",*b"CERES",*b"CERGE",*b"CERIA",*b"CERIC",*b"CERNE",*b"CEROC",*b"CEROS",*b"CERTS",
	*b"CERTY",*b"CESSE",*b"CESTA",*b"CESTI",*b"CETES",*b"CETYL",*b"CEZVE",*b"CHACE",*b"CHACK",*b"CHACO",
	*b"CHADO",*b"CHADS",*b"CHAFE",*b"CHAFF",*b"CHAFT",*b"CHAIN",*b"CHAIR",*b"CHAIS",*b"CHALK",*b"CHALS",
	*b"CHAMP",*b"CHAMS",*b"CHANA",*b"CHANG",*b"CHANK",*b"CHANT",*b"CHAOS",*b"CHAPE",*b"CHAPS",*b"CHAPT",
	*b"CHARA",*b"CHARD",*b"CHARE",*b"CHARK",*b"CHARM",*b"CHARR",*b"CHARS",*b"CHART",*b"CHARY",*b"CHASE",
	*b"CHASM",*b"CHATS",*b"CHAVE",*b"CHAVS",*b"CHAWK",*b"CHAWS",*b"CHAYA",*b"CHAYS",*b"CHEAP",*b"CHEAT",
	*b"CHECK",*b"CHEEK",*b"CHEEP",*b"CHEER",*b"CHEFS",*b"CHEKA",*b"CHELA",*b"CHELP",*b"CHEMO",*b"CHEMS",
	*b"CHERE",*b"CHERT",*b"CHESS",*b"CHEST",*b"CHETH",*b"CHEVY",*b"CHEWS",*b"CHEWY",*b"CHIAO",*b"CHIAS",
	*b"CHIBS",*b"CHICA",*b"CHICH",*b"CHICK",*b"CHICO",*b"CHICS",*b"CHIDE",*b"CHIEF",*b"CHIEL",*b"CHIKS",
	*b"CHILD",*b"CHILE",*b"CHILI",*b"CHILL",*b"CHIMB",*b"CHIME",*b"CHIMO",*b"CHIMP",*b"CHINA",*b"CHINE",
	*b"CHING",*b"CHINK",*b"CHINO",*b"CHINS",*b"CHIPS",*b"CHIRK",*b"CHIRL",*b"CHIRM",*b"CHIRO",*b"CHIRP",
	*b"CHIRR",*b"CHIRT",*b"CHIRU",*b"CHITS",*b"CHIVE",*b"CHIVS",*b"CHIVY",*b"CHIZZ",*b"CHOCK",*b"CHOCO",
	*b"CHOCS",*b"CHODE",*b"CHOGS",*b"CHOIL",*b"CHOIR",*b"CHOKE",*b"CHOKO",*b"CHOKY",*b"CHOLA",*b"CHOLI",
	*b"CHOLO",*b"CHOMP",*b"CHONS",*b"CHOOF",*b"CHOOK",*b"CHOOM",*b"CHOON",*b"CHOPS",*b"CHORD",*b"CHORE",
	*b"CHOSE",*b"CHOTA",*b"CHOTT",*b"CHOUT",*b"CHOUX",*b"CHOWK",*b"CHOWS",*b"CHUBS",*b"CHUCK",*b"CHUFA",
	*b"CHUFF",*b"CHUGS",*b"CHUMP",*b"CHUMS",*b"CHUNK",*b"CHURL",*b"CHURN",*b"CHURR",*b"CHUSE",*b"CHUTE",
	*b"CHUTS",*b"CHYLE",*b"CHYME",*b"CHYND",*b"CIBOL",*b"CIDED",*b"CIDER",*b"CIDES",*b"CIELS",*b"CIGAR",
	*b"CIGGY",*b"CILIA",*b"CILLS",*b"CIMAR",*b"CIMEX",*b"CINCH",*b"CINCT",*b"CINES",*b"CINQS",*b"CIONS",
	*b"CIPPI",*b"CIRCA",*b"CIRCS",*b"CIRES",*b"CIRLS",*b"CIRRI",*b"CISCO",*b"CISSY",*b"CISTS",*b"CITAL",
	*b"CITED",*b"CITER",*b"CITES",*b"CIVES",*b"CIVET",*b"CIVIC",*b"CIVIE",*b"CIVIL",*b"CIVVY",*b"CLACH",
	*b"CLACK",*b"CLADE",*b"CLADS",*b"CLAES",*b"CLAGS",*b"CLAIM",*b"CLAME",*b"CLAMP",*b"CLAMS",*b"CLANG",
	*b"CLANK",*b"CLANS",*b"CLAPS",*b"CLAPT",*b"CLARO",*b"CLART",*b"CLARY",*b"CLASH",*b"CLASP",*b"CLASS",
	*b"CLAST",*b"CLATS",*b"CLAUT",*b"CLAVE",*b"CLAVI",*b"CLAWS",*b"CLAYS",*b"CLEAN",*b"CLEAR",*b"CLEAT",
	*b"CLECK",*b"CLEEK",*b"CLEEP",*b"CLEFS",*b"CLEFT",*b"CLEGS",*b"CLEIK",*b"CLEMS",*b"CLEPE",*b"CLEPT",
	*b"CLERK",*b"CLEVE",*b"CLEWS",*b"CLICK",*b"CLIED",*b"CLIES",*b"CLIFF",*b"CLIFT",*b"CLIMB",*b"CLIME",
	*b"CLINE",*b"CLING",*b"CLINK",*b"CLINT",*b"CLIPE",*b"CLIPS",*b"CLIPT",*b"CLITS",*b"CLOAK",*b"CLOAM",
	*b"CLOCK",*b"CLODS",*b"CLOFF",*b"CLOGS",*b"CLOKE",*b"CLOMB",*b"CLOMP",*b"CLONE",*b"CLONK",*b"CLONS",
	*b"CLOOP",*b"CLOOT",*b"CLOPS",*b"CLOSE",*b"CLOTE",*b"CLOTH",*b"CLOTS",*b"CLOUD",*b"CLOUR",*b"CLOUS",
	*b"CLOUT",*b"CLOVE",*b"CLOWN",*b"CLOWS",*b"CLOYE",*b"CLOYS",*b"CLOZE",*b"CLUBS",*b"CLUCK",*b"CLUED",
	*b"CLUES",*b"CLUEY",*b"CLUMP",*b"CLUNG",*b"CLUNK",*b"CLYPE",*b"CNIDA",*b"COACH",*b"COACT",*b"COADY",
	*b"COALA",*b"COALS",*b"COALY",*b"COAPT",*b"COARB",*b"COAST",*b"COATE",*b"COATI",*b"COATS",*b"COBBS",
	*b"COBBY",*b"COBIA",*b"COBLE",*b"COBRA",*b"COBZA",*b"COCAS",*b"COCCI",*b"COCCO",*b"COCKS",*b"COCKY",
	*b"COCOA",*b"COCOS",*b"CODAS",*b"CODEC",*b"CODED",*b"CODEN",*b"CODER",*b"CODES",*b"CODEX",*b"CODON",
	*b"COEDS",*b"COFFS",*b"COGIE",*b"COGON",*b"COGUE",*b"COHAB",*b"COHEN",*b"COHOE",*b"COHOG",*b"COHOS",
	*b"COIFS",*b"COIGN",*b"COILS",*b"COINS",*b"COIRS",*b"COITS",*b"COKED",*b"COKES",*b"COLAS",*b"COLBY",
	*b"COLDS",*b"COLED",*b"COLES",*b"COLEY",*b"COLIC",*b"COLIN",*b"COLLS",*b"COLLY",*b"COLOG",*b"COLON",
	*b"COLOR",*b"COLTS",*b"COLZA",*b"COMAE",*b"COMAL",*b"COMAS",*b"COMBE",*b"COMBI",*b"COMBO",*b"COMBS",
	*b"COMBY",*b"COMER",*b"COMES",*b"COMET",*b"COMFY",*b"COMIC",*b"COMIX",*b"COMMA",*b"COMMO",*b"COMMS",
	*b"COMMY",*b"COMPO",*b"COMPS",*b"COMPT",*b"COMTE",*b"COMUS",*b"CONCH",*b"CONDO",*b"CONED",*b"CONES",
	*b"CONEY",*b"CONFS",*b"CONGA",*b"CONGE",*b"CONGO",*b"CONIA",*b"CONIC",*b"CONIN",*b"CONKS",*b"CONKY",
	*b"CONNE",*b"CONNS",*b"CONTE",*b"CONTO",*b"CONUS",*b"CONVO",*b"COOCH",*b"COOED",*b"COOEE",*b"COOER",
	*b"COOEY",*b"COOFS",*b"COOKS",*b"COOKY",*b"COOLS",*b"COOLY",*b"COOMB",*b"COOMS",*b"COOMY",*b"COONS",
	*b"COOPS",*b"COOPT",*b"COOST",*b"COOTS",*b"COOZE",*b"COPAL",*b"COPAY",*b"COPED",*b"COPEN",*b"COPER",
	*b"COPES",*b"COPPY",*b"COPRA",*b"COPSE",*b"COPSY",*b"COQUI",*b"CORAL",*b"CORAM",*b"CORBE",*b"CORBY",
	*b"CORDS",*b"CORED",*b"CORER",*b"CORES",*b"COREY",*b"CORGI",*b"CORIA",*b"CORKS",*b"CORKY",*b"CORMS",
	*b"CORNI",*b"CORNO",*b"CORNS",*b"CORNU",*b"CORNY",*b"CORPS",*b"CORSE",*b"CORSO",*b"COSEC",*b"COSED",
	*b"COSES",*b"COSET",*b"COSEY",*b"COSIE",*b"COSTA",*b"COSTE",*b"COSTS",*b"COTAN",*b"COTED",*b"COTES",
	*b"COTHS",*b"COTTA",*b"COTTS",*b"COUCH",*b"COUDE",*b"COUGH",*b"COULD",*b"COUNT",*b"COUPE",*b"COUPS",
	*b"COURB",*b"COURD",*b"COURE",*b"COURS",*b"COURT",*b"COUTA",*b"COUTH",*b"COVED",*b"COVEN",*b"COVER",
	*b"COVES",*b"COVET",*b"COVEY",*b"COVIN",*b"COWAL",*b"COWAN",*b"COWED",*b"COWER",*b"COWKS",*b"COWLS",
	*b"COWPS",*b"COWRY",*b"COXAE",*b"COXAL",*b"COXED",*b"COXES",*b"COXIB",*b"COYAU",*b"COYED",*b"COYER",
	*b"COYLY",*b"COYPU",*b"COZED",*b"COZEN",*b"COZES",*b"COZEY",*b"COZIE",*b"CRAAL",*b"CRABS",*b"CRACK",
	*b"CRAFT",*b"CRAGS",*b"CRAIC",*b"CRAIG",*b"CRAKE",*b"CRAME",*b"CRAMP",*b"CRAMS",*b"CRANE",*b"CRANK",
	*b"CRANS",*b"CRAPE",*b"CRAPS",*b"CRAPY",*b"CRARE",*b"CRASH",*b"CRASS",*b"CRATE",*b"CRAVE",*b"CRAWL",
	*b"CRAWS",*b"CRAYS",*b"CRAZE",*b"CRAZY",*b"CREAK",*b"CREAM",*b"CREDO",*b"CREDS",*b"CREED",*b"CREEK",
	*b"CREEL",*b"CREEP",*b"CREES",*b"CREME",*b"CREMS",*b"CRENA",*b"CREPE",*b"CREPS",*b"CREPT",*b"CREPY",
	*b"CRESS",*b"CREST",*b"CREWE",*b"CREWS",*b"CRIAS",*b"CRIBS",*b"CRICK",*b"CRIED",*b"CRIER",*b"CRIES",
	*b"CRIME",*b"CRIMP",*b"CRIMS",*b"CRINE",*b"CRIOS",*b"CRIPE",*b"CRIPS",*b"CRISE",*b"CRISP",*b"CRITH",
	*b"CRITS",*b"CROAK",*b"CROCI",*b"CROCK",*b"CROCS",*b"CROFT",*b"CROGS",*b"CROMB",*b"CROME",*b"CRONE",
	*b"CRONK",*b"CRONS",*b"CRONY",*b"CROOK",*b"CROOL",*b"CROON",*b"CROPS",*b"CRORE",*b"CROSS",*b"CROST",
	*b"CROUP",*b"CROUT",*b"CROWD",*b"CROWN",*b"CROWS",*b"CROZE",*b"CRUCK",*b"CRUDE",*b"CRUDO",*b"CRUDS",
	*b"CRUDY",*b"CRUEL",*b"CRUES",*b"CRUET",*b"CRUFT",*b"CRUMB",*b"CRUMP",*b"CRUNK",*b"CRUOR",*b"CRURA",
	*b"CRUSE",*b"CRUSH",*b"CRUST",*b"CRUSY",*b"CRUVE",*b"CRWTH",*b"CRYER",*b"CRYPT",*b"CTENE",*b"CUBBY",
	*b"CUBEB",*b"CUBED",*b"CUBER",*b"CUBES",*b"CUBIC",*b"CUBIT",*b"CUDDY",*b"CUFFO",*b"CUFFS",*b"CUIFS",
	*b"CUING",*b"CUISH",*b"CUITS",*b"CUKES",*b"CULCH",*b"CULET",*b"CULEX",*b"CULLS",*b"CULLY",*b"CULMS",
	*b"CULPA",*b"CULTI",*b"CULTS",*b"CULTY",*b"CUMEC",*b"CUMIN",*b"CUNDY",*b"CUNEI",*b"CUNIT",*b"CUNTS",
	*b"CUPEL",*b"CUPID",*b"CUPPA",*b"CUPPY",*b"CURAT",*b"CURBS",*b"CURCH",*b"CURDS",*b"CURDY",*b"CURED",
	*b"CURER",*b"CURES",*b"CURET",*b"CURFS",*b"CURIA",*b"CURIE",*b"CURIO",*b"CURLI",*b"CURLS",*b"CURLY",
	*b"CURNS",*b"CURNY",*b"CURRS",*b"CURRY",*b"CURSE",*b"CURSI",*b"CURST",*b"CURVE",*b"CURVY",*b"CUSEC",
	*b"CUSHY",*b"CUSKS",*b"CUSPS",*b"CUSPY",*b"CUSSO",*b"CUSUM",*b"CUTCH",*b"CUTER",*b"CUTES",*b"CUTEY",
	*b"CUTIE",*b"CUTIN",*b"CUTIS",*b"CUTTO",*b"CUTTY",*b"CUTUP",*b"CUVEE",*b"CUZES",*b"CWTCH",*b"CYANO",
	*b"CYANS",*b"CYBER",*b"CYCAD",*b"CYCAS",*b"CYCLE",*b"CYCLO",*b"CYDER",*b"CYLIX",*b"CYMAE",*b"CYMAR",
	*b"CYMAS",*b"CYMES",*b"CYMOL",*b"CYNIC",*b"CYSTS",*b"CYTES",*b"CYTON",*b"CZARS",*b"DAALS",*b"DABBA",
	*b"DACES",*b"DACHA",*b"DACKS",*b"DADAH",*b"DADAS",*b"DADDY",*b"DADOS",*b"DAFFS",*b"DAFFY",*b"DAGGA",
	*b"DAGGY",*b"DAGOS",*b"DAHLS",*b"DAIKO",*b"DAILY",*b"DAINE",*b"DAINT",*b"DAIRY",*b"DAISY",*b"DAKER",
	*b"DALED",*b"DALES",*b"DALIS",*b"DALLE",*b"DALLY",*b"DALTS",*b"DAMAN",*b"DAMAR",*b"DAMES",*b"DAMME",
	*b"DAMNS",*b"DAMPS",*b"DAMPY",*b"DANCE",*b"DANCY",*b"DANDY",*b"DANGS",*b"DANIO",*b"DANKS",*b"DANNY",
	*b"DANTS",*b"DARAF",*b"DARBS",*b"DARCY",*b"DARED",*b"DARER",*b"DARES",*b"DARGA",*b"DARGS",*b"DARIC",
	*b"DARIS",*b"DARKS",*b"DARKY",*b"DARNS",*b"DARRE",*b"DARTS",*b"DARZI",*b"DASHI",*b"DASHY",*b"DATAL",
	*b"DATED",*b"DATER",*b"DATES",*b"DATOS",*b"DATTO",*b"DATUM",*b"DAUBE",*b"DAUBS",*b"DAUBY",*b"DAUDS",
	*b"DAULT",*b"DAUNT",*b"DAURS",*b"DAUTS",*b"DAVEN",*b"DAVIT",*b"DAWAH",*b"DAWDS",*b"DAWED",*b"DAWEN",
	*b"DAWKS",*b"DAWNS",*b"DAWTS",*b"DAYAN",*b"DAYCH",*b"DAYNT",*b"DAZED",*b"DAZER",*b"DAZES",*b"DEADS",
	*b"DEAIR",*b"DEALS",*b"DEALT",*b"DEANS",*b"DEARE",*b"DEARN",*b"DEARS",*b"DEARY",*b"DEASH",*b"DEATH",
	*b"DEAVE",*b"DEAWS",*b"DEAWY",*b"DEBAG",*b"DEBAR",*b"DEBBY",*b"DEBEL",*b"DEBES",*b"DEBIT",*b"DEBTS",
	*b"DEBUD",*b"DEBUG",*b"DEBUR",*b"DEBUS",*b"DEBUT",*b"DEBYE",*b"DECAD",*b"DECAF",*b"DECAL",*b"DECAN",
	*b"DECAY",*b"DECKO",*b"DECKS",*b"DECOR",*b"DECOS",*b"DECOY",*b"DECRY",*b"DEDAL",*b"DEEDS",*b"DEEDY",
	*b"DEELY",*b"DEEMS",*b"DEENS",*b"DEEPS",*b"DEERE",*b"DEERS",*b"DEETS",*b"DEEVE",*b"DEEVS",*b"DEFAT",
	*b"DEFER",*b"DEFFO",*b"DEFIS",*b"DEFOG",*b"DEGAS",*b"DEGUM",*b"DEGUS",*b"DEICE",*b"DEIDS",*b"DEIFY",
	*b"DEIGN",*b"DEILS",*b"DEISM",*b"DEIST",*b"DEITY",*b"DEKED",*b"DEKES",*b"DEKKO",*b"DELAY",*b"DELED",
	*b"DELES",*b"DELFS",*b"DELFT",*b"DELIS",*b"DELLS",*b"DELLY",*b"DELOS",*b"DELPH",*b"DELTA",*b"DELTS",
	*b"DELVE",*b"DEMAN",*b"DEMES",*b"DEMIC",*b"DEMIT",*b"DEMOB",*b"DEMOI",*b"DEMON",*b"DEMOS",*b"DEMPT",
	*b"DEMUR",*b"DENAR",*b"DENAY",*b"DENCH",*b"DENES",*b"DENET",*b"DENIM",*b"DENIS",*b"DENSE",*b"DENTS",
	*b"DEOXY",*b"DEPOT",*b"DEPTH",*b"DERAT",*b"DERAY",*b"DERBY",*b"DERED",*b"DERES",*b"DERIG",*b"DERMA",
	*b"DERMS",*b"DERNS",*b"DERNY",*b"DEROS",*b"DERRO",*b"DERRY",*b"DERTH",*b"DERVS",*b"DESEX",*b"DESHI",
	*b"DESIS",*b"DESKS",*b"DESSE",*b"DETER",*b"DETOX",*b"DEUCE",*b"DEVAS",*b"DEVEL",*b"DEVIL",*b"DEVIS",
	*b"DEVON",*b"DEVOS",*b"DEVOT",*b"DEWAN",*b"DEWAR",*b"DEWAX",*b"DEWED",*b"DEXES",*b"DEXIE",*b"DHABA",
	*b"DHAKS",*b"DHALS",*b"DHIKR",*b"DHOBI",*b"DHOLE",*b"DHOLL",*b"DHOLS",*b"DHOTI",*b"DHOWS",*b"DHUTI",
	*b"DIACT",*b"DIALS",*b"DIANE",*b"DIARY",*b"DIAZO",*b"DIBBS",*b"DICED",*b"DICER",*b"DICES",*b"DICEY",
	*b"DICHT",*b"DICKS",*b"DICKY",*b"DICOT",*b"DICTA",*b"DICTS",*b"DICTY",*b"DIDDY",*b"DIDIE",*b"DIDOS",
	*b"DIDST",*b"DIEBS",*b"DIELS",*b"DIENE",*b"DIETS",*b"DIFFS",*b"DIGHT",*b"DIGIT",*b"DIKAS",*b"DIKED",
	*b"DIKER",*b"DIKES",*b"DIKEY",*b"DILDO",*b"DILLI",*b"DILLS",*b"DILLY",*b"DIMBO",*b"DIMER",*b"DIMES",
	*b"DIMLY",*b"DIMPS",*b"DINAR",*b"DINED",*b"DINER",*b"DINES",*b"DINGE",*b"DINGO",*b"DINGS",*b"DINGY",
	*b"DINIC",*b"DINKS",*b"DINKY",*b"DINNA",*b"DINOS",*b"DINTS",*b"DIODE",*b"DIOLS",*b"DIOTA",*b"DIPPY",
	*b"DIPSO",*b"DIRAM",*b"DIRER",*b"DIRGE",*b"DIRKE",*b"DIRKS",*b"DIRLS",*b"DIRTS",*b"DIRTY",*b"DISAS",
	*b"DISCI",*b"DISCO",*b"DISCS",*b"DISHY",*b"DISKS",*b"DISME",*b"DITAL",*b"DITAS",*b"DITCH",*b"DITED",
	*b"DITES",*b"DITSY",*b"DITTO",*b"DITTS",*b"DITTY",*b"DITZY",*b"DIVAN",*b"DIVAS",*b"DIVED",*b"DIVER",
	*b"DIVES",*b"DIVIS",*b"DIVNA",*b"DIVOS",*b"DIVOT",*b"DIVVY",*b"DIWAN",*b"DIXIE",*b"DIXIT",*b"DIYAS",
	*b"DIZEN",*b"DIZZY",*b"DJINN",*b"DJINS",*b"DOABS",*b"DOATS",*b"DOBBY",*b"DOBES",*b"DOBIE",*b"DOBLA",
	*b"DOBRA",*b"DOBRO",*b"DOCHT",*b"DOCKS",*b"DOCOS",*b"DOCUS",*b"DODDY",*b"DODGE",*b"DODGY",*b"DODOS",
	*b"DOEKS",*b"DOERS",*b"DOEST",*b"DOETH",*b"DOFFS",*b"DOGAN",*b"DOGES",*b"DOGEY",*b"DOGGO",*b"DOGGY",
	*b"DOGIE",*b"DOGMA",*b"DOHYO",*b"DOILT",*b"DOILY",*b"DOING",*b"DOITS",*b"DOJOS",*b"DOLCE",*b"DOLCI",
	*b"DOLED",*b"DOLES",*b"DOLIA",*b"DOLLS",*b"DOLLY",*b"DOLMA",*b"DOLOR",*b"DOLOS",*b"DOLTS",*b"DOMAL",
	*b"DOMED",*b"DOMES",*b"DOMIC",*b"DONAH",*b"DONAS",*b"DONEE",*b"DONER",*b"DONGA",*b"DONGS",*b"DONKO",
	*b"DONNA",*b"DONNE",*b"DONNY",*b"DONOR",*b"DONSY",*b"DONUT",*b"DOOBS",*b"DOOCE",*b"DOODY",*b"DOOKS",
	*b"DOOLE",*b"DOOLS",*b"DOOLY",*b"DOOMS",*b"DOOMY",*b"DOONA",*b"DOORN",*b"DOORS",*b"DOOZY",*b"DOPAS",
	*b"DOPED",*b"DOPER",*b"DOPES",*b"DOPEY",*b"DORAD",*b"DORBA",*b"DORBS",*b"DOREE",*b"DORES",*b"DORIC",
	*b"DORIS",*b"DORKS",*b"DORKY",*b"DORMS",*b"DORMY",*b"DORPS",*b"DORRS",*b"DORSA",*b"DORSE",*b"DORTS",
	*b"DORTY",*b"DOSAI",*b"DOSAS",*b"DOSED",*b"DOSEH",*b"DOSER",*b"DOSES",*b"DOSHA",*b"DOTAL",*b"DOTED",
	*b"DOTER",*b"DOTES",*b"DOTTY",*b"DOUAR",*b"DOUBT",*b"DOUCE",*b"DOUCS",*b"DOUGH",*b"DOUKS",*b"DOULA",
	*b"DOUMA",*b"DOUMS",*b"DOUPS",*b"DOURA",*b"DOUSE",*b"DOUTS",*b"DOVED",*b"DOVEN",*b"DOVER",*b"DOVES",
	*b"DOVIE",*b"DOWAR",*b"DOWDS",*b"DOWDY",*b"DOWED",*b"DOWEL",*b"DOWER",*b"DOWIE",*b"DOWLE",*b"DOWLS",
	*b"DOWLY",*b"DOWNA",*b"DOWNS",*b"DOWNY",*b"DOWPS",*b"DOWRY",*b"DOWSE",*b"DOWTS",*b"DOXED",*b"DOXES",
	*b"DOXIE",*b"DOYEN",*b"DOYLY",*b"DOZED",*b"DOZEN",*b"DOZER",*b"DOZES",*b"DRABS",*b"DRACK",*b"DRACO",
	*b"DRAFF",*b"DRAFT",*b"DRAGS",*b"DRAIL",*b"DRAIN",*b"DRAKE",*b"DRAMA",*b"DRAMS",*b"DRANK",*b"DRANT",
	*b"DRAPE",*b"DRAPS",*b"DRATS",*b"DRAVE",*b"DRAWL",*b"DRAWN",*b"DRAWS",*b"DRAYS",*b"DREAD",*b"DREAM",
	*b"DREAR",*b"DRECK",*b"DREED",*b"DREER",*b"DREES",*b"DREGS",*b"DREKS",*b"DRENT",*b"DRERE",*b"DRESS",
	*b"DREST",*b"DREYS",*b"DRIBS",*b"DRICE",*b"DRIED",*b"DRIER",*b"DRIES",*b"DRIFT",*b"DRILL",*b"DRILY",
	*b"DRINK",*b"DRIPS",*b"DRIPT",*b"DRIVE",*b"DROID",*b"DROIL",*b"DROIT",*b"DROKE",*b"DROLE",*b"DROLL",
	*b"DROME",*b"DRONE",*b"DRONY",*b"DROOB",*b"DROOG",*b"DROOK",*b"DROOL",*b"DROOP",*b"DROPS",*b"DROPT",
	*b"DROSS",*b"DROUK",*b"DROVE",*b"DROWN",*b"DROWS",*b"DRUBS",*b"DRUGS",*b"DRUID",*b"DRUMS",*b"DRUNK",
	*b"DRUPE",*b"DRUSE",*b"DRUSY",*b"DRUXY",*b"DRYAD",*b"DRYAS",*b"DRYER",*b"DRYLY",*b"DSOBO",*b"DSOMO",
	*b"DUADS",*b"DUALS",*b"DUANS",*b"DUARS",*b"DUBBO",*b"DUCAL",*b"DUCAT",*b"DUCES",*b"DUCHY",*b"DUCKS",
	*b"DUCKY",*b"DUCTS",*b"DUDDY",*b"DUDED",*b"DUDES",*b"DUELS",*b"DUETS",*b"DUETT",*b"DUFFS",*b"DUFUS",
	*b"DUING",*b"DUITS",*b"DUKAS",*b"DUKED",*b"DUKES",*b"DUKKA",*b"DULCE",*b"DULES",*b"DULIA",*b"DULLS",
	*b"DULLY",*b"DULSE",*b"DUMAS",*b"DUMBO",*b"DUMBS",*b"DUMKA",*b"DUMKY",*b"DUMMY",*b"DUMPS",*b"DUMPY",
	*b"DUNAM",*b"DUNCE",*b"DUNCH",*b"DUNES",*b"DUNGS",*b"DUNGY",*b"DUNKS",*b"DUNNO",*b"DUNNY",*b"DUNSH",
	*b"DUNTS",*b"DUOMI",*b"DUOMO",*b"DUPED",*b"DUPER",*b"DUPES",*b"DUPLE",*b"DUPLY",*b"DUPPY",*b"DURAL",
	*b"DURAS",*b"DURED",*b"DURES",*b"DURGY",*b"DURNS",*b"DUROC",*b"DUROS",*b"DUROY",*b"DURRA",*b"DURRS",
	*b"DURRY",*b"DURST",*b"DURUM",*b"DURZI",*b"DUSKS",*b"DUSKY",*b"DUSTS",*b"DUSTY",*b"DUTCH",*b"DUVET",
	*b"DUXES",*b"DWAAL",*b"DWALE",*b"DWALM",*b"DWAMS",*b"DWANG",*b"DWARF",*b"DWAUM",*b"DWEEB",*b"DWELL",
	*b"DWELT",*b"DWILE",*b"DWINE",*b"DYADS",*b"DYERS",*b"DYING",*b"DYKED",*b"DYKES",*b"DYKEY",*b"DYKON",
	*b"DYNEL",*b"DYNES",*b"DZHOS",*b"EAGER",*b"EAGLE",*b"EAGRE",*b"EALED",*b"EALES",*b"EANED",*b"EARDS",
	*b"EARED",*b"EARLS",*b"EARLY",*b"EARNS",*b"EARNT",*b"EARST",*b"EARTH",*b"EASED",*b"EASEL",*b"EASER",
	*b"EASES",*b"EASLE",*b"EASTS",*b"EATEN",*b"EATER",*b"EATHE",*b"EAVED",*b"EAVES",*b"EBBED",*b"EBBET",
	*b"EBONS",*b"EBONY",*b"EBOOK",*b"ECADS",*b"ECHED",*b"ECHES",*b"ECHOS",*b"ECLAT",*b"ECRUS",*b"EDEMA",
	*b"EDGED",*b"EDGER",*b"EDGES",*b"EDICT",*b"EDIFY",*b"EDILE",*b"EDITS",*b"EDUCE",*b"EDUCT",*b"EEJIT",
	*b"EENSY",*b"EERIE",*b"EEVEN",*b"EEVNS",*b"EFFED",*b"EGADS",*b"EGERS",*b"EGEST",*b"EGGAR",*b"EGGED",
	*b"EGGER",*b"EGMAS",*b"EGRET",*b"EHING",*b"EIDER",*b"EIDOS",*b"EIGHT",*b"EIGNE",*b"EIKED",*b"EIKON",
	*b"EILDS",*b"EISEL",*b"EJECT",*b"EJIDO",*b"EKING",*b"EKKAS",*b"ELAIN",*b"ELAND",*b"ELANS",*b"ELATE",
	*b"ELBOW",*b"ELCHI",*b"ELDER",*b"ELDIN",*b"ELECT",*b"ELEGY",*b"ELEMI",*b"ELFED",*b"ELFIN",*b"ELIAD",
	*b"ELIDE",*b"ELINT",*b"ELITE",*b"ELMEN",*b"ELOGE",*b"ELOGY",*b"ELOIN",*b"ELOPE",*b"ELOPS",*b"ELPEE",
	*b"ELSIN",*b"ELUDE",*b"ELUTE",*b"ELVAN",*b"ELVEN",*b"ELVER",*b"ELVES",*b"EMACS",*b"EMAIL",*b"EMBAR",
	*b"EMBAY",*b"EMBED",*b"EMBER",*b"EMBOG",*b"EMBOW",*b"EMBOX",*b"EMBUS",*b"EMCEE",*b"EMEER",*b"EMEND",
	*b"EMERG",*b"EMERY",*b"EMEUS",*b"EMICS",*b"EMIRS",*b"EMITS",*b"EMMAS",*b"EMMER",*b"EMMET",*b"EMMEW",
	*b"EMMYS",*b"EMOJI",*b"EMONG",*b"EMOTE",*b"EMOVE",*b"EMPTS",*b"EMPTY",*b"EMULE",*b"EMURE",*b"EMYDE",
	*b"EMYDS",*b"ENACT",*b"ENARM",*b"ENATE",*b"ENDED",*b"ENDER",*b"ENDEW",*b"ENDOW",*b"ENDUE",*b"ENEMA",
	*b"ENEMY",*b"ENEWS",*b"ENFIX",*b"ENIAC",*b"ENJOY",*b"ENLIT",*b"ENMEW",*b"ENNOG",*b"ENNUI",*b"ENOKI",
	*b"ENOLS",*b"ENORM",*b"ENOWS",*b"ENROL",*b"ENSEW",*b"ENSKY",*b"ENSUE",*b"ENTER",*b"ENTIA",*b"ENTRY",
	*b"ENURE",*b"ENURN",*b"ENVOI",*b"ENVOY",*b"ENZYM",*b"EORLS",*b"EOSIN",*b"EPACT",*b"EPEES",*b"EPHAH",
	*b"EPHAS",*b"EPHOD",*b"EPHOR",*b"EPICS",*b"EPOCH",*b"EPODE",*b"EPOPT",*b"EPOXY",*b"EPRIS",*b"EQUAL",
	*b"EQUES",*b"EQUID",*b"EQUIP",*b"ERASE",*b"ERBIA",*b"ERECT",*b"EREVS",*b"ERGON",*b"ERGOS",*b"ERGOT",
	*b"ERHUS",*b"ERICA",*b"ERICK",*b"ERICS",*b"ERING",*b"ERNED",*b"ERNES",*b"ERODE",*b"EROSE",*b"ERRED",
	*b"ERROR",*b"ERSES",*b"ERUCT",*b"ERUGO",*b"ERUPT",*b"ERUVS",*b"ERVEN",*b"ERVIL",*b"ESCAR",*b"ESCOT",
	*b"ESILE",*b"ESKAR",*b"ESKER",*b"ESNES",*b"ESSAY",*b"ESSES",*b"ESTER",*b"ESTOC",*b"ESTOP",*b"ESTRO",
	*b"ETAGE",*b"ETAPE",*b"ETATS",*b"ETENS",*b"ETHAL",*b"ETHER",*b"ETHIC",*b"ETHNE",*b"ETHOS",*b"ETHYL",
	*b"ETICS",*b"ETNAS",*b"ETTIN",*b"ETTLE",*b"ETUDE",*b"ETUIS",*b"ETWEE",*b"ETYMA",*b"EUGHS",*b"EUKED",
	*b"EUPAD",*b"EUROS",*b"EUSOL",*b"EVADE",*b"EVENS",*b"EVENT",*b"EVERT",*b"EVERY",*b"EVETS",*b"EVHOE",
	*b"EVICT",*b"EVILS",*b"EVITE",*b"EVOHE",*b"EVOKE",*b"EWERS",*b"EWEST",*b"EWHOW",*b"EWKED",*b"EXACT",
	*b"EXALT",*b"EXAMS",*b"EXCEL",*b"EXEAT",*b"EXECS",*b"EXEEM",*b"EXEME",*b"EXERT",*b"EXFIL",*b"EXIES",
	*b"EXILE",*b"EXINE",*b"EXING",*b"EXIST",*b"EXITS",*b"EXODE",*b"EXOME",*b"EXONS",*b"EXPAT",*b"EXPEL",
	*b"EXPOS",*b"EXTOL",*b"EXTRA",*b"EXUDE",*b"EXULS",*b"EXULT",*b"EXURB",*b"EYASS",*b"EYERS",*b"EYING",
	*b"EYOTS",*b"EYRAS",*b"EYRES",*b"EYRIE",*b"EYRIR",*b"EZINE",*b"FABBY",*b"FABLE",*b"FACED",*b"FACER",
	*b"FACES",*b"FACET",*b"FACIA",*b"FACTA",*b"FACTS",*b"FADDY",*b"FADED",*b"FADER",*b"FADES",*b"FADGE",
	*b"FADOS",*b"FAENA",*b"FAERY",*b"FAFFS",*b"FAFFY",*b"FAGGY",*b"FAGIN",*b"FAGOT",*b"FAIKS",*b"FAILS",
	*b"FAINE",*b"FAINS",*b"FAINT",*b"FAIRS",*b"FAIRY",*b"FAITH",*b"FAKED",*b"FAKER",*b"FAKES",*b"FAKEY",
	*b"FAKIE",*b"FAKIR",*b"FALAJ",*b"FALLS",*b"FALSE",*b"FAMED",*b"FAMES",*b"FANAL",*b"FANCY",*b"FANDS",
	*b"FANES",*b"FANGA",*b"FANGO",*b"FANGS",*b"FANKS",*b"FANNY",*b"FANON",*b"FANOS",*b"FANUM",*b"FAQIR",
	*b"FARAD",*b"FARCE",*b"FARCI",*b"FARCY",*b"FARDS",*b"FARED",*b"FARER",*b"FARES",*b"FARLE",*b"FARLS",
	*b"FARMS",*b"FAROS",*b"FARRO",*b"FARSE",*b"FARTS",*b"FASCI",*b"FASTI",*b"FASTS",*b"FATAL",*b"FATED",
	*b"FATES",*b"FATLY",*b"FATSO",*b"FATTY",*b"FATWA",*b"FAUGH",*b"FAULD",*b"FAULT",*b"FAUNA",*b"FAUNS",
	*b"FAURD",*b"FAUTS",*b"FAUVE",*b"FAVAS",*b"FAVEL",*b"FAVER",*b"FAVES",*b"FAVOR",*b"FAVUS",*b"FAWNS",
	*b"FAWNY",*b"FAXED",*b"FAXES",*b"FAYED",*b"FAYER",*b"FAYNE",*b"FAYRE",*b"FAZED",*b"FAZES",*b"FEALS",
	*b"FEARE",*b"FEARS",*b"FEART",*b"FEASE",*b"FEAST",*b"FEATS",*b"FEAZE",*b"FECAL",*b"FECES",*b"FECHT",
	*b"FECIT",*b"FECKS",*b"FEDEX",*b"FEEBS",*b"FEEDS",*b"FEELS",*b"FEENS",*b"FEERS",*b"FEESE",*b"FEEZE",
	*b"FEHME",*b"FEIGN",*b"FEINT",*b"FEIST",*b"FELCH",*b"FELID",*b"FELLA",*b"FELLS",*b"FELLY",*b"FELON",
	*b"FELTS",*b"FELTY",*b"FEMAL",*b"FEMES",*b"FEMME",*b"FEMMY",*b"FEMUR",*b"FENCE",*b"FENDS",*b"FENDY",
	*b"FENIS",*b"FENKS",*b"FENNY",*b"FENTS",*b"FEODS",*b"FEOFF",*b"FERAL",*b"FERER",*b"FERES",*b"FERIA",
	*b"FERLY",*b"FERMI",*b"FERMS",*b"FERNS",*b"FERNY",*b"FERRY",*b"FESSE",*b"FESTA",*b"FESTS",*b"FESTY",
	*b"FETAL",*b"FETAS",*b"FETCH",*b"FETED",*b"FETES",*b"FETID",*b"FETOR",*b"FETTA",*b"FETTS",*b"FETUS",
	*b"FETWA",*b"FEUAR",*b"FEUDS",*b"FEUED",*b"FEVER",*b"FEWER",*b"FEYED",*b"FEYER",*b"FEYLY",*b"FEZES",
	*b"FEZZY",*b"FIARS",*b"FIATS",*b"FIBER",*b"FIBRE",*b"FIBRO",*b"FICES",*b"FICHE",*b"FICHU",*b"FICIN",
	*b"FICOS",*b"FICUS",*b"FIDES",*b"FIDGE",*b"FIDOS",*b"FIEFS",*b"FIELD",*b"FIEND",*b"FIENT",*b"FIERE",
	*b"FIERS",*b"FIERY",*b"FIEST",*b"FIFED",*b"FIFER",*b"FIFES",*b"FIFIS",*b"FIFTH",*b"FIFTY",*b"FIGGY",
	*b"FIGHT",*b"FIGOS",*b"FIKED",*b"FIKES",*b"FILAR",*b"FILCH",*b"FILED",*b"FILER",*b"FILES",*b"FILET",
	*b"FILII",*b"FILKS",*b"FILLE",*b"FILLO",*b"FILLS",*b"FILLY",*b"FILMI",*b"FILMS",*b"FILMY",*b"FILOS",
	*b"FILTH",*b"FILUM",*b"FINAL",*b"FINCA",*b"FINCH",*b"FINDS",*b"FINED",*b"FINER",*b"FINES",*b"FINIS",
	*b"FINKS",*b"FINNY",*b"FINOS",*b"FIORD",*b"FIQHS",*b"FIQUE",*b"FIRED",*b"FIRER",*b"FIRES",*b"FIRIE",
	*b"FIRKS",*b"FIRMS",*b"FIRNS",*b"FIRRY",*b"FIRST",*b"FIRTH",*b"FISCS",*b"FISHY",*b"FISKS",*b"FISTS",
	*b"FISTY",*b"FITCH",*b"FITLY",*b"FITNA",*b"FITTE",*b"FITTS",*b"FIVER",*b"FIVES",*b"FIXED",*b"FIXER",
	*b"FIXES",*b"FIXIT",*b"FIZZY",*b"FJELD",*b"FJORD",*b"FLABS",*b"FLACK",*b"FLAFF",*b"FLAGS",*b"FLAIL",
	*b"FLAIR",*b"FLAKE",*b"FLAKS",*b"FLAKY",*b"FLAME",*b"FLAMM",*b"FLAMS",*b"FLAMY",*b"FLANE",*b"FLANK",
	*b"FLANS",*b"FLAPS",*b"FLARE",*b"FLARY",*b"FLASH",*b"FLASK",*b"FLATS",*b"FLAVA",*b"FLAWN",*b"FLAWS",
	*b"FLAWY",*b"FLAXY",*b"FLAYS",*b"FLEAM",*b"FLEAS",*b"FLECK",*b"FLEEK",*b"FLEER",*b"FLEES",*b"FLEET",
	*b"FLEGS",*b"FLEME",*b"FLESH",*b"FLEUR",*b"FLEWS",*b"FLEXI",*b"FLEXO",*b"FLEYS",*b"FLICK",*b"FLICS",
	*b"FLIED",*b"FLIER",*b"FLIES",*b"FLIMP",*b"FLIMS",*b"FLING",*b"FLINT",*b"FLIPS",*b"FLIRS",*b"FLIRT",
	*b"FLISK",*b"FLITE",*b"FLITS",*b"FLITT",*b"FLOAT",*b"FLOBS",*b"FLOCK",*b"FLOCS",*b"FLOES",*b"FLOGS",
	*b"FLONG",*b"FLOOD",*b"FLOOR",*b"FLOPS",*b"FLORA",*b"FLORS",*b"FLORY",*b"FLOSH",*b"FLOSS",*b"FLOTA",
	*b"FLOTE",*b"FLOUR",*b"FLOUT",*b"FLOWN",*b"FLOWS",*b"FLUBS",*b"FLUED",*b"FLUES",*b"FLUEY",*b"FLUFF",
	*b"FLUID",*b"FLUKE",*b"FLUKY",*b"FLUME",*b"FLUMP",*b"FLUNG",*b"FLUNK",*b"FLUOR",*b"FLURR",*b"FLUSH",
	*b"FLUTE",*b"FLUTY",*b"FLUYT",*b"FLYBY",*b"FLYER",*b"FLYPE",*b"FLYTE",*b"FOALS",*b"FOAMS",*b"FOAMY",
	*b"FOCAL",*b"FOCUS",*b"FOEHN",*b"FOGEY",*b"FOGGY",*b"FOGIE",*b"FOGLE",*b"FOGOU",*b"FOHNS",*b"FOIDS",
	*b"FOILS",*b"FOINS",*b"FOIST",*b"FOLDS",*b"FOLEY",*b"FOLIA",*b"FOLIC",*b"FOLIE",*b"FOLIO",*b"FOLKS",
	*b"FOLKY",*b"FOLLY",*b"FOMES",*b"FONDA",*b"FONDS",*b"FONDU",*b"FONES",*b"FONLY",*b"FONTS",*b"FOODS",
	*b"FOODY",*b"FOOLS",*b"FOOTS",*b"FOOTY",*b"FORAM",*b"FORAY",*b"FORBS",*b"FORBY",*b"FORCE",*b"FORDO",
	*b"FORDS",*b"FOREL",*b"FORES",*b"FOREX",*b"FORGE",*b"FORGO",*b"FORKS",*b"FORKY",*b"FORME",*b"FORMS",
	*b"FORTE",*b"FORTH",*b"FORTS",*b"FORTY",*b"FORUM",*b"FORZA",*b"FORZE",*b"FOSSA",*b"FOSSE",*b"FOUAT",
	*b"FOUDS",*b"FOUER",*b"FOUET",*b"FOULE",*b"FOULS",*b"FOUND",*b"FOUNT",*b"FOURS",*b"FOUTH",*b"FOVEA",
	*b"FOWLS",*b"FOWTH",*b"FOXED",*b"FOXES",*b"FOXIE",*b"FOYER",*b"FOYLE",*b"FOYNE",*b"FRABS",*b"FRACK",
	*b"FRACT",*b"FRAGS",*b"FRAIL",*b"FRAIM",*b"FRAME",*b"FRANC",*b"FRANK",*b"FRAPE",*b"FRAPS",*b"FRASS",
	*b"FRATE",*b"FRATI",*b"FRATS",*b"FRAUD",*b"FRAUS",*b"FRAYS",*b"FREAK",*b"FREED",*b"FREER",*b"FREES",
	*b"FREET",*b"FREIT",*b"FREMD",*b"FRENA",*b"FREON",*b"FRERE",*b"FRESH",*b"FRETS",*b"FRIAR",*b"FRIBS",
	*b"FRIED",*b"FRIER",*b"FRIES",*b"FRIGS",*b"FRILL",*b"FRISE",*b"FRISK",*b"FRIST",*b"FRITH",*b"FRITS",
	*b"FRITT",*b"FRITZ",*b"FRIZE",*b"FRIZZ",*b"FROCK",*b"FROES",*b"FROGS",*b"FROND",*b"FRONS",*b"FRONT",
	*b"FRORE",*b"FRORN",*b"FRORY",*b"FROSH",*b"FROST",*b"FROTH",*b"FROWN",*b"FROWS",*b"FROWY",*b"FROZE",
	*b"FRUGS",*b"FRUIT",*b"FRUMP",*b"FRUSH",*b"FRUST",*b"FRYER",*b"FUBAR",*b"FUBBY",*b"FUBSY",*b"FUCKS",
	*b"FUCUS",*b"FUDDY",*b"FUDGE",*b"FUDGY",*b"FUELS",*b"FUERO",*b"FUFFS",*b"FUFFY",*b"FUGAL",*b"FUGGY",
	*b"FUGIE",*b"FUGIO",*b"FUGLE",*b"FUGLY",*b"FUGUE",*b"FUGUS",*b"FUJIS",*b"FULLS",*b"FULLY",*b"FUMED",
	*b"FUMER",*b"FUMES",*b"FUMET",*b"FUNDI",*b"FUNDS",*b"FUNDY",*b"FUNGI",*b"FUNGO",*b"FUNGS",*b"FUNKS",
	*b"FUNKY",*b"FUNNY",*b"FURAL",*b"FURAN",*b"FURCA",*b"FURLS",*b"FUROL",*b"FUROR",*b"FURRS",*b"FURRY",
	*b"FURTH",*b"FURZE",*b"FURZY",*b"FUSED",*b"FUSEE",*b"FUSEL",*b"FUSES",*b"FUSIL",*b"FUSKS",*b"FUSSY",
	*b"FUSTS",*b"FUSTY",*b"FUTON",*b"FUZED",*b"FUZEE",*b"FUZES",*b"FUZIL",*b"FUZZY",*b"FYCES",*b"FYKED",
	*b"FYKES",*b"FYLES",*b"FYRDS",*b"FYTTE",*b"GABBA",*b"GABBY",*b"GABLE",*b"GADDI",*b"GADES",*b"GADGE",
	*b"GADID",*b"GADIS",*b"GADJE",*b"GADJO",*b"GADSO",*b"GAFFE",*b"GAFFS",*b"GAGED",*b"GAGER",*b"GAGES",
	*b"GAIDS",*b"GAILY",*b"GAINS",*b"GAIRS",*b"GAITA",*b"GAITS",*b"GAITT",*b"GAJOS",*b"GALAH",*b"GALAS",
	*b"GALAX",*b"GALEA",*b"GALED",*b"GALES",*b"GALLS",*b"GALLY",*b"GALOP",*b"GALUT",*b"GALVO",*b"GAMAS",
	*b"GAMAY",*b"GAMBA",*b"GAMBE",*b"GAMBO",*b"GAMBS",*b"GAMED",*b"GAMER",*b"GAMES",*b"GAMEY",*b"GAMIC",
	*b"GAMIN",*b"GAMMA",*b"GAMME",*b"GAMMY",*b"GAMPS",*b"GAMUT",*b"GANCH",*b"GANDY",*b"GANEF",*b"GANEV",
	*b"GANGS",*b"GANJA",*b"GANOF",*b"GANTS",*b"GAOLS",*b"GAPED",*b"GAPER",*b"GAPES",*b"GAPOS",*b"GAPPY",
	*b"GARBE",*b"GARBO",*b"GARBS",*b"GARDA",*b"GARES",*b"GARIS",*b"GARMS",*b"GARNI",*b"GARRE",*b"GARTH",
	*b"GARUM",*b"GASES",*b"GASPS",*b"GASPY",*b"GASSY",*b"GASTS",*b"GATCH",*b"GATED",*b"GATER",*b"GATES",
	*b"GATHS",*b"GATOR",*b"GAUCH",*b"GAUCY",*b"GAUDS",*b"GAUDY",*b"GAUGE",*b"GAUJE",*b"GAULT",*b"GAUMS",
	*b"GAUMY",*b"GAUNT",*b"GAUPS",*b"GAURS",*b"GAUSS",*b"GAUZE",*b"GAUZY",*b"GAVEL",*b"GAVOT",*b"GAWCY",
	*b"GAWDS",*b"GAWKS",*b"GAWKY",*b"GAWPS",*b"GAWSY",*b"GAYAL",*b"GAYER",*b"GAYLY",*b"GAZAL",*b"GAZAR",
	*b"GAZED",*b"GAZER",*b"GAZES",*b"GAZON",*b"GAZOO",*b"GEALS",*b"GEANS",*b"GEARE",*b"GEARS",*b"GEATS",
	*b"GEBUR",*b"GECKO",*b"GECKS",*b"GEEKS",*b"GEEKY",*b"GEEPS",*b"GEESE",*b"GEEST",*b"GEIST",*b"GEITS",
	*b"GELDS",*b"GELEE",*b"GELID",*b"GELLY",*b"GELTS",*b"GEMEL",*b"GEMMA",*b"GEMMY",*b"GEMOT",*b"GENAL",
	*b"GENAS",*b"GENES",*b"GENET",*b"GENIC",*b"GENIE",*b"GENII",*b"GENIP",*b"GENNY",*b"GENOA",*b"GENOM",
	*b"GENRE",*b"GENRO",*b"GENTS",*b"GENTY",*b"GENUA",*b"GENUS",*b"GEODE",*b"GEOID",*b"GERAH",*b"GERBE",
	*b"GERES",*b"GERLE",*b"GERMS",*b"GERMY",*b"GERNE",*b"GESSE",*b"GESSO",*b"GESTE",*b"GESTS",*b"GETAS",
	*b"GETUP",*b"GEUMS",*b"GEYAN",*b"GEYER",*b"GHAST",*b"GHATS",*b"GHAUT",*b"GHAZI",*b"GHEES",*b"GHEST",
	*b"GHOST",*b"GHOUL",*b"GHYLL",*b"GIANT",*b"GIBED",*b"GIBEL",*b"GIBER",*b"GIBES",*b"GIBLI",*b"GIBUS",
	*b"GIDDY",*b"GIFTS",*b"GIGAS",*b"GIGHE",*b"GIGOT",*b"GIGUE",*b"GILAS",*b"GILDS",*b"GILET",*b"GILLS",
	*b"GILLY",*b"GILPY",*b"GILTS",*b"GIMEL",*b"GIMME",*b"GIMPS",*b"GIMPY",*b"GINCH",*b"GINGE",*b"GINGS",
	*b"GINKS",*b"GINNY",*b"GINZO",*b"GIPON",*b"GIPPO",*b"GIPPY",*b"GIPSY",*b"GIRDS",*b"GIRLS",*b"GIRLY",
	*b"GIRNS",*b"GIRON",*b"GIROS",*b"GIRRS",*b"GIRSH",*b"GIRTH",*b"GIRTS",*b"GISMO",*b"GISMS",*b"GISTS",
	*b"GITCH",*b"GITES",*b"GIUST",*b"GIVED",*b"GIVEN",*b"GIVER",*b"GIVES",*b"GIZMO",*b"GLACE",*b"GLADE",
	*b"GLADS",*b"GLADY",*b"GLAIK",*b"GLAIR",*b"GLAMS",*b"GLAND",*b"GLANS",*b"GLARE",*b"GLARY",*b"GLASS",
	*b"GLAUM",*b"GLAUR",*b"GLAZE",*b"GLAZY",*b"GLEAM",*b"GLEAN",*b"GLEBA",*b"GLEBE",*b"GLEBY",*b"GLEDE",
	*b"GLEDS",*b"GLEED",*b"GLEEK",*b"GLEES",*b"GLEET",*b"GLEIS",*b"GLENS",*b"GLENT",*b"GLEYS",*b"GLIAL",
	*b"GLIAS",*b"GLIBS",*b"GLIDE",*b"GLIFF",*b"GLIFT",*b"GLIKE",*b"GLIME",*b"GLIMS",*b"GLINT",*b"GLISK",
	*b"GLITS",*b"GLITZ",*b"GLOAM",*b"GLOAT",*b"GLOBE",*b"GLOBI",*b"GLOBS",*b"GLOBY",*b"GLODE",*b"GLOGG",
	*b"GLOMS",*b"GLOOM",*b"GLOOP",*b"GLOPS",*b"GLORY",*b"GLOSS",*b"GLOST",*b"GLOUT",*b"GLOVE",*b"GLOWS",
	*b"GLOZE",*b"GLUED",*b"GLUER",*b"GLUES",*b"GLUEY",*b"GLUGS",*b"GLUME",*b"GLUMS",*b"GLUON",*b"GLUTE",
	*b"GLUTS",*b"GLYPH",*b"GNARL",*b"GNARR",*b"GNARS",*b"GNASH",*b"GNATS",*b"GNAWN",*b"GNAWS",*b"GNOME",
	*b"GNOWS",*b"GOADS",*b"GOAFS",*b"GOALS",*b"GOARY",*b"GOATS",*b"GOATY",*b"GOBAN",*b"GOBAR",*b"GOBBI",
	*b"GOBBO",*b"GOBBY",*b"GOBIS",*b"GOBOS",*b"GODET",*b"GODLY",*b"GODSO",*b"GOELS",*b"GOERS",*b"GOEST",
	*b"GOETH",*b"GOETY",*b"GOFER",*b"GOFFS",*b"GOGGA",*b"GOGOS",*b"GOIER",*b"GOING",*b"GOJIS",*b"GOLDS",
	*b"GOLDY",*b"GOLEM",*b"GOLES",*b"GOLFS",*b"GOLLY",*b"GOLPE",*b"GOLPS",*b"GOMBO",*b"GOMER",*b"GOMPA",
	*b"GONAD",*b"GONCH",*b"GONEF",*b"GONER",*b"GONGS",*b"GONIA",*b"GONIF",*b"GONKS",*b"GONNA",*b"GONOF",
	*b"GONYS",*b"GONZO",*b"GOOBY",*b"GOODS",*b"GOODY",*b"GOOEY",*b"GOOFS",*b"GOOFY",*b"GOOGS",*b"GOOKS",
	*b"GOOKY",*b"GOOLD",*b"GOOLS",*b"GOOLY",*b"GOONS",*b"GOONY",*b"GOOPS",*b"GOOPY",*b"GOORS",*b"GOORY",
	*b"GOOSE",*b"GOOSY",*b"GOPAK",*b"GOPIK",*b"GORAL",*b"GORAS",*b"GORED",*b"GORES",*b"GORGE",*b"GORIS",
	*b"GORMS",*b"GORMY",*b"GORPS",*b"GORSE",*b"GORSY",*b"GOSHT",*b"GOSSE",*b"GOTCH",*b"GOTHS",*b"GOTHY",
	*b"GOTTA",*b"GOUCH",*b"GOUGE",*b"GOUKS",*b"GOURA",*b"GOURD",*b"GOUTS",*b"GOUTY",*b"GOWAN",*b"GOWDS",
	*b"GOWFS",*b"GOWKS",*b"GOWLS",*b"GOWNS",*b"GOXES",*b"GOYIM",*b"GOYLE",*b"GRAAL",*b"GRABS",*b"GRACE",
	*b"GRADE",*b"GRADS",*b"GRAFF",*b"GRAFT",*b"GRAIL",*b"GRAIN",*b"GRAIP",*b"GRAMA",*b"GRAME",*b"GRAMP",
	*b"GRAMS",*b"GRANA",*b"GRAND",*b"GRANS",*b"GRANT",*b"GRAPE",*b"GRAPH",*b"GRAPY",*b"GRASP",*b"GRASS",
	*b"GRATE",*b"GRAVE",*b"GRAVS",*b"GRAVY",*b"GRAYS",*b"GRAZE",*b"GREAT",*b"GREBE",*b"GREBO",*b"GRECE",
	*b"GREED",*b"GREEK",*b"GREEN",*b"GREES",*b"GREET",*b"GREGE",*b"GREGO",*b"GREIN",*b"GRENS",*b"GRESE",
	*b"GREVE",*b"GREWS",*b"GREYS",*b"GRICE",*b"GRIDE",*b"GRIDS",*b"GRIEF",*b"GRIFF",*b"GRIFT",*b"GRIGS",
	*b"GRIKE",*b"GRILL",*b"GRIME",*b"GRIMY",*b"GRIND",*b"GRINS",*b"GRIOT",*b"GRIPE",*b"GRIPS",*b"GRIPT",
	*b"GRIPY",*b"GRISE",*b"GRIST",*b"GRISY",*b"GRITH",*b"GRITS",*b"GRIZE",*b"GROAN",*b"GROAT",*b"GRODY",
	*b"GROGS",*b"GROIN",*b"GROKS",*b"GROMA",*b"GRONE",*b"GROOF",*b"GROOM",*b"GROPE",*b"GROSS",*b"GROSZ",
	*b"GROTS",*b"GROUF",*b"GROUP",*b"GROUT",*b"GROVE",*b"GROVY",*b"GROWL",*b"GROWN",*b"GROWS",*b"GRRLS",
	*b"GRRRL",*b"GRUBS",*b"GRUED",*b"GRUEL",*b"GRUES",*b"GRUFE",*b"GRUFF",*b"GRUME",*b"GRUMP",*b"GRUND",
	*b"GRUNT",*b"GRYCE",*b"GRYDE",*b"GRYKE",*b"GRYPE",*b"GRYPT",*b"GUACO",*b"GUANA",*b"GUANO",*b"GUANS",
	*b"GUARD",*b"GUARS",*b"GUAVA",*b"GUCKS",*b"GUCKY",*b"GUDES",*b"GUESS",*b"GUEST",*b"GUFFS",*b"GUGAS",
	*b"GUIDE",*b"GUIDS",*b"GUILD",*b"GUILE",*b"GUILT",*b"GUIMP",*b"GUIRO",*b"GUISE",*b"GULAG",*b"GULAR",
	*b"GULAS",*b"GULCH",*b"GULES",*b"GULET",*b"GULFS",*b"GULFY",*b"GULLS",*b"GULLY",*b"GULPH",*b"GULPS",
	*b"GULPY",*b"GUMBO",*b"GUMMA",*b"GUMMI",*b"GUMMY",*b"GUMPS",*b"GUNDY",*b"GUNGE",*b"GUNGY",*b"GUNKS",
	*b"GUNKY",*b"GUNNY",*b"GUPPY",*b"GUQIN",*b"GURDY",*b"GURGE",*b"GURLS",*b"GURLY",*b"GURNS",*b"GURRY",
	*b"GURSH",*b"GURUS",*b"GUSHY",*b"GUSLA",*b"GUSLE",*b"GUSLI",*b"GUSSY",*b"GUSTO",*b"GUSTS",*b"GUSTY",
	*b"GUTSY",*b"GUTTA",*b"GUTTY",*b"GUYED",*b"GUYLE",*b"GUYOT",*b"GUYSE",*b"GWINE",*b"GYALS",*b"GYANS",
	*b"GYBED",*b"GYBES",*b"GYELD",*b"GYMPS",*b"GYNAE",*b"GYNIE",*b"GYNNY",*b"GYNOS",*b"GYOZA",*b"GYPOS",
	*b"GYPPO",*b"GYPPY",*b"GYPSY",*b"GYRAL",*b"GYRED",*b"GYRES",*b"GYRON",*b"GYROS",*b"GYRUS",*b"GYTES",
	*b"GYVED",*b"GYVES",*b"HAAFS",*b"HAARS",*b"HABIT",*b"HABLE",*b"HABUS",*b"HACEK",*b"HACKS",*b"HADAL",
	*b"HADED",*b"HADES",*b"HADJI",*b"HADST",*b"HAEMS",*b"HAETS",*b"HAFFS",*b"HAFIZ",*b"HAFTS",*b"HAGGS",
	*b"HAHAS",*b"HAICK",*b"HAIKA",*b"HAIKS",*b"HAIKU",*b"HAILS",*b"HAILY",*b"HAINS",*b"HAINT",*b"HAIRS",
	*b"HAIRY",*b"HAITH",*b"HAJES",*b"HAJIS",*b"HAJJI",*b"HAKAM",*b"HAKAS",*b"HAKEA",*b"HAKES",*b"HAKIM",
	*b"HAKUS",*b"HALAL",*b"HALED",*b"HALER",*b"HALES",*b"HALFA",*b"HALFS",*b"HALID",*b"HALLO",*b"HALLS",
	*b"HALMA",*b"HALMS",*b"HALON",*b"HALOS",*b"HALSE",*b"HALTS",*b"HALVA",*b"HALVE",*b"HALWA",*b"HAMAL",
	*b"HAMBA",*b"HAMED",*b"HAMES",*b"HAMMY",*b"HAMZA",*b"HANAP",*b"HANCE",*b"HANCH",*b"HANDS",*b"HANDY",
	*b"HANGI",*b"HANGS",*b"HANKS",*b"HANKY",*b"HANSA",*b"HANSE",*b"HANTS",*b"HAOLE",*b"HAOMA",*b"HAPAX",
	*b"HAPLY",*b"HAPPI",*b"HAPPY",*b"HAPUS",*b"HARAM",*b"HARDS",*b"HARDY",*b"HARED",*b"HAREM",*b"HARES",
	*b"HARIM",*b"HARKS",*b"HARLS",*b"HARMS",*b"HARNS",*b"HAROS",*b"HARPS",*b"HARPY",*b"HARRY",*b"HARSH",
	*b"HARTS",*b"HASHY",*b"HASKS",*b"HASPS",*b"HASTA",*b"HASTE",*b"HASTY",*b"HATCH",*b"HATED",*b"HATER",
	*b"HATES",*b"HATHA",*b"HAUDS",*b"HAUFS",*b"HAUGH",*b"HAULD",*b"HAULM",*b"HAULS",*b"HAULT",*b"HAUNS",
	*b"HAUNT",*b"HAUSE",*b"HAUTE",*b"HAVEN",*b"HAVER",*b"HAVES",*b"HAVOC",*b"HAWED",*b"HAWKS",*b"HAWMS",
	*b"HAWSE",*b"HAYED",*b"HAYER",*b"HAYEY",*b"HAYLE",*b"HAZAN",*b"HAZED",*b"HAZEL",*b"HAZER",*b"HAZES",
	*b"HEADS",*b"HEADY",*b"HEALD",*b"HEALS",*b"HEAME",*b"HEAPS",*b"HEAPY",*b"HEARD",*b"HEARE",*b"HEARS",
	*b"HEART",*b"HEAST",*b"HEATH",*b"HEATS",*b"HEAVE",*b"HEAVY",*b"HEBEN",*b"HEBES",*b"HECHT",*b"HECKS",
	*b"HEDER",*b"HEDGE",*b"HEDGY",*b"HEEDS",*b"HEEDY",*b"HEELS",*b"HEEZE",*b"HEFTE",*b"HEFTS",*b"HEFTY",
	*b"HEIDS",*b"HEIGH",*b"HEILS",*b"HEIRS",*b"HEIST",*b"HEJAB",*b"HEJRA",*b"HELED",*b"HELES",*b"HELIO",
	*b"HELIX",*b"HELLO",*b"HELLS",*b"HELMS",*b"HELOS",*b"HELOT",*b"HELPS",*b"HELVE",*b"HEMAL",*b"HEMES",
	*b"HEMIC",*b"HEMIN",*b"HEMPS",*b"HEMPY",*b"HENCE",*b"HENCH",*b"HENDS",*b"HENGE",*b"HENNA",*b"HENNY",
	*b"HENRY",*b"HENTS",*b"HEPAR",*b"HERBS",*b"HERBY",*b"HERDS",*b"HERES",*b"HERLS",*b"HERMA",*b"HERMS",
	*b"HERNS",*b"HERON",*b"HEROS",*b"HERRY",*b"HERSE",*b"HERTZ",*b"HERYE",*b"HESPS",*b"HESTS",*b"HETES",
	*b"HETHS",*b"HEUCH",*b"HEUGH",*b"HEVEA",*b"HEWED",*b"HEWER",*b"HEWGH",*b"HEXAD",*b"HEXED",*b"HEXER",
	*b"HEXES",*b"HEXYL",*b"HEYED",*b"HIANT",*b"HICKS",*b"HIDED",*b"HIDER",*b"HIDES",*b"HIEMS",*b"HIGHS",
	*b"HIGHT",*b"HIJAB",*b"HIJRA",*b"HIKED",*b"HIKER",*b"HIKES",*b"HIKOI",*b"HILAR",*b"HILCH",*b"HILLO",
	*b"HILLS",*b"HILLY",*b"HILTS",*b"HILUM",*b"HILUS",*b"HIMBO",*b"HINAU",*b"HINDS",*b"HINGE",*b"HINGS",
	*b"HINKY",*b"HINNY",*b"HINTS",*b"HIOIS",*b"HIPLY",*b"HIPPO",*b"HIPPY",*b"HIRED",*b"HIREE",*b"HIRER",
	*b"HIRES",*b"HISSY",*b"HISTS",*b"HITCH",*b"HITHE",*b"HIVED",*b"HIVER",*b"HIVES",*b"HIZEN",*b"HOAED",
	*b"HOAGY",*b"HOARD",*b"HOARS",*b"HOARY",*b"HOAST",*b"HOBBY",*b"HOBOS",*b"HOCKS",*b"HOCUS",*b"HODAD",
	*b"HODJA",*b"HOERS",*b"HOGAN",*b"HOGEN",*b"HOGGS",*b"HOGHS",*b"HOHED",*b"HOICK",*b"HOIED",*b"HOIKS",
	*b"HOING",*b"HOISE",*b"HOIST",*b"HOKAS",*b"HOKED",*b"HOKES",*b"HOKEY",*b"HOKIS",*b"HOKKU",*b"HOKUM",
	*b"HOLDS",*b"HOLED",*b"HOLES",*b"HOLEY",*b"HOLKS",*b"HOLLA",*b"HOLLO",*b"HOLLY",*b"HOLME",*b"HOLMS",
	*b"HOLON",*b"HOLOS",*b"HOLTS",*b"HOMAS",*b"HOMED",*b"HOMER",*b"HOMES",*b"HOMEY",*b"HOMIE",*b"HOMME",
	*b"HOMOS",*b"HONAN",*b"HONDA",*b"HONDS",*b"HONED",*b"HONER",*b"HONES",*b"HONEY",*b"HONGI",*b"HONGS",
	*b"HONKS",*b"HONKY",*b"HONOR",*b"HOOCH",*b"HOODS",*b"HOODY",*b"HOOEY",*b"HOOFS",*b"HOOKA",*b"HOOKS",
	*b"HOOKY",*b"HOOLY",*b"HOONS",*b"HOOPS",*b"HOORD",*b"HOORS",*b"HOOSH",*b"HOOTS",*b"HOOTY",*b"HOOVE",
	*b"HOPAK",*b"HOPED",*b"HOPER",*b"HOPES",*b"HOPPY",*b"HORAH",*b"HORAL",*b"HORAS",*b"HORDE",*b"HORIS",
	*b"HORKS",*b"HORME",*b"HORNS",*b"HORNY",*b"HORSE",*b"HORST",*b"HORSY",*b"HOSED",*b"HOSEL",*b"HOSEN",
	*b"HOSER",*b"HOSES",*b"HOSEY",*b"HOSTA",*b"HOSTS",*b"HOTCH",*b"HOTEL",*b"HOTEN",*b"HOTLY",*b"HOTTY",
	*b"HOUFF",*b"HOUFS",*b"HOUGH",*b"HOUND",*b"HOURI",*b"HOURS",*b"HOUSE",*b"HOUTS",*b"HOVEA",*b"HOVED",
	*b"HOVEL",*b"HOVEN",*b"HOVER",*b"HOVES",*b"HOWBE",*b"HOWDY",*b"HOWES",*b"HOWFF",*b"HOWFS",*b"HOWKS",
	*b"HOWLS",*b"HOWRE",*b"HOWSO",*b"HOXED",*b"HOXES",*b"HOYAS",*b"HOYED",*b"HOYLE",*b"HUBBY",*b"HUCKS",
	*b"HUDNA",*b"HUDUD",*b"HUERS",*b"HUFFS",*b"HUFFY",*b"HUGER",*b"HUGGY",*b"HUHUS",*b"HUIAS",*b"HULAS",
	*b"HULES",*b"HULKS",*b"HULKY",*b"HULLO",*b"HULLS",*b"HULLY",*b"HUMAN",*b"HUMAS",*b"HUMFS",*b"HUMIC",
	*b"HUMID",*b"HUMOR",*b"HUMPH",*b"HUMPS",*b"HUMPY",*b"HUMUS",*b"HUNCH",*b"HUNKS",*b"HUNKY",*b"HUNTS",
	*b"HURDS",*b"HURLS",*b"HURLY",*b"HURRA",*b"HURRY",*b"HURST",*b"HURTS",*b"HUSHY",*b"HUSKS",*b"HUSKY",
	*b"HUSOS",*b"HUSSY",*b"HUTCH",*b"HUTIA",*b"HUZZA",*b"HUZZY",*b"HWYLS",*b"HYDRA",*b"HYDRO",*b"HYENA",
	*b"HYENS",*b"HYGGE",*b"HYING",*b"HYKES",*b"HYLAS",*b"HYLEG",*b"HYLES",*b"HYLIC",*b"HYMEN",*b"HYMNS",
	*b"HYNDE",*b"HYOID",*b"HYPED",*b"HYPER",*b"HYPES",*b"HYPHA",*b"HYPHY",*b"HYPOS",*b"HYRAX",*b"HYSON",
	*b"HYTHE",*b"IAMBI",*b"IAMBS",*b"IBRIK",*b"ICERS",*b"ICHED",*b"ICHES",*b"ICHOR",*b"ICIER",*b"ICILY",
	*b"ICING",*b"ICKER",*b"ICKLE",*b"ICONS",*b"ICTAL",*b"ICTIC",*b"ICTUS",*b"IDANT",*b"IDEAL",*b"IDEAS",
	*b"IDEES",*b"IDENT",*b"IDIOM",*b"IDIOT",*b"IDLED",*b"IDLER",*b"IDLES",*b"IDOLA",*b"IDOLS",*b"IDYLL",
	*b"IDYLS",*b"IFTAR",*b"IGAPO",*b"IGGED",*b"IGLOO",*b"IGLUS",*b"IHRAM",*b"IKANS",*b"IKATS",*b"IKONS",
	*b"ILEAC",*b"ILEAL",*b"ILEUM",*b"ILEUS",*b"ILIAC",*b"ILIAD",*b"ILIAL",*b"ILIUM",*b"ILLER",*b"ILLTH",
	*b"IMAGE",*b"IMAGO",*b"IMAMS",*b"IMARI",*b"IMAUM",*b"IMBAR",*b"IMBED",*b"IMBUE",*b"IMIDE",*b"IMIDO",
	*b"IMIDS",*b"IMINE",*b"IMINO",*b"IMMEW",*b"IMMIT",*b"IMMIX",*b"IMPED",*b"IMPEL",*b"IMPIS",*b"IMPLY",
	*b"IMPOT",*b"IMPRO",*b"IMSHI",*b"IMSHY",*b"INANE",*b"INAPT",*b"INARM",*b"INBOX",*b"INBYE",*b"INCEL",
	*b"INCLE",*b"INCOG",*b"INCUR",*b"INCUS",*b"INCUT",*b"INDEW",*b"INDEX",*b"INDIA",*b"INDIE",*b"INDOL",
	*b"INDOW",*b"INDRI",*b"INDUE",*b"INEPT",*b"INERM",*b"INERT",*b"INFER",*b"INFIX",*b"INFOS",*b"INFRA",
	*b"INGAN",*b"INGLE",*b"INGOT",*b"INION",*b"INKED",*b"INKER",*b"INKLE",*b"INLAY",*b"INLET",*b"INNED",
	*b"INNER",*b"INNIT",*b"INORB",*b"INPUT",*b"INRUN",*b"INSET",*b"INSPO",*b"INTEL",*b"INTER",*b"INTIL",
	*b"INTIS",*b"INTRA",*b"INTRO",*b"INULA",*b"INURE",*b"INURN",*b"INUST",*b"INVAR",*b"INWIT",*b"IODIC",
	*b"IODID",*b"IODIN",*b"IONIC",*b"IOTAS",*b"IPPON",*b"IRADE",*b"IRATE",*b"IRIDS",*b"IRING",*b"IRKED",
	*b"IROKO",*b"IRONE",*b"IRONS",*b"IRONY",*b"ISBAS",*b"ISHES",*b"ISLED",*b"ISLES",*b"ISLET",*b"ISNAE",
	*b"ISSEI",*b"ISSUE",*b"ISTLE",*b"ITCHY",*b"ITEMS",*b"ITHER",*b"IVIED",*b"IVIES",*b"IVORY",*b"IXIAS",
	*b"IXNAY",*b"IXORA",*b"IXTLE",*b"IZARD",*b"IZARS",*b"IZZAT",*b"JAAPS",*b"JABOT",*b"JACAL",*b"JACKS",
	*b"JACKY",*b"JADED",*b"JADES",*b"JAFAS",*b"JAFFA",*b"JAGAS",*b"JAGER",*b"JAGGS",*b"JAGGY",*b"JAGIR",
	*b"JAGRA",*b"JAILS",*b"JAKER",*b"JAKES",*b"JAKEY",*b"JALAP",*b"JALOP",*b"JAMBE",*b"JAMBO",*b"JAMBS",
	*b"JAMBU",*b"JAMES",*b"JAMMY",*b"JAMON",*b"JANES",*b"JANNS",*b"JANNY",*b"JANTY",*b"JAPAN",*b"JAPED",
	*b"JAPER",*b"JAPES",*b"JARKS",*b"JARLS",*b"JARPS",*b"JARTA",*b"JARUL",*b"JASEY",*b"JASPE",*b"JASPS",
	*b"JATOS",*b"JAUKS",*b"JAUNT",*b"JAUPS",*b"JAVAS",*b"JAVEL",*b"JAWAN",*b"JAWED",*b"JAXIE",*b"JAZZY",
	*b"JEANS",*b"JEATS",*b"JEBEL",*b"JEDIS",*b"JEELS",*b"JEELY",*b"JEEPS",*b"JEERS",*b"JEEZE",*b"JEFES",
	*b"JEFFS",*b"JEHAD",*b"JEHUS",*b"JELAB",*b"JELLO",*b"JELLS",*b"JELLY",*b"JEMBE",*b"JEMMY",*b"JENNY",
	*b"JEONS",*b"JERID",*b"JERKS",*b"JERKY",*b"JERRY",*b"JESSE",*b"JESTS",*b"JESUS",*b"JETES",*b"JETON",
	*b"JETTY",*b"JEUNE",*b"JEWED",*b"JEWEL",*b"JEWIE",*b"JHALA",*b"JIAOS",*b"JIBBA",*b"JIBBS",*b"JIBED",
	*b"JIBER",*b"JIBES",*b"JIFFS",*b"JIFFY",*b"JIGGY",*b"JIGOT",*b"JIHAD",*b"JILLS",*b"JILTS",*b"JIMMY",
	*b"JIMPY",*b"JINGO",*b"JINKS",*b"JINNE",*b"JINNI",*b"JINNS",*b"JIRDS",*b"JIRGA",*b"JIRRE",*b"JISMS",
	*b"JIVED",*b"JIVER",*b"JIVES",*b"JIVEY",*b"JNANA",*b"JOBED",*b"JOBES",*b"JOCKO",*b"JOCKS",*b"JOCKY",
	*b"JOCOS",*b"JODEL",*b"JOEYS",*b"JOHNS",*b"JOINS",*b"JOINT",*b"JOIST",*b"JOKED",*b"JOKER",*b"JOKES",
	*b"JOKEY",*b"JOKOL",*b"JOLED",*b"JOLES",*b"JOLLS",*b"JOLLY",*b"JOLTS",*b"JOLTY",*b"JOMON",*b"JOMOS",
	*b"JONES",*b"JONGS",*b"JONTY",*b"JOOKS",*b"JORAM",*b"JORUM",*b"JOTAS",*b"JOTTY",*b"JOTUN",*b"JOUAL",
	*b"JOUGS",*b"JOUKS",*b"JOULE",*b"JOURS",*b"JOUST",*b"JOWAR",*b"JOWED",*b"JOWLS",*b"JOWLY",*b"JOYED",
	*b"JUBAS",*b"JUBES",*b"JUCOS",*b"JUDAS",*b"JUDGE",*b"JUDGY",*b"JUDOS",*b"JUGAL",*b"JUGUM",*b"JUICE",
	*b"JUICY",*b"JUJUS",*b"JUKED",*b"JUKES",*b"JUKUS",*b"JULEP",*b"JUMAR",*b"JUMBO",*b"JUMBY",*b"JUMPS",
	*b"JUMPY",*b"JUNCO",*b"JUNKS",*b"JUNKY",*b"JUNTA",*b"JUNTO",*b"JUPES",*b"JUPON",*b"JURAL",*b"JURAT",
	*b"JUREL",*b"JURES",*b"JUROR",*b"JUSTS",*b"JUTES",*b"JUTTY",*b"JUVES",*b"JUVIE",*b"KAAMA",*b"KABAB",
	*b"KABAR",*b"KABOB",*b"KACHA",*b"KACKS",*b"KADAI",*b"KADES",*b"KADIS",*b"KAFIR",*b"KAGOS",*b"KAGUS",
	*b"KAHAL",*b"KAIAK",*b"KAIDS",*b"KAIES",*b"KAIFS",*b"KAIKA",*b"KAIKS",*b"KAILS",*b"KAIMS",*b"KAING",
	*b"KAINS",*b"KAKAS",*b"KAKIS",*b"KALAM",*b"KALES",*b"KALIF",*b"KALIS",*b"KALPA",*b"KAMAS",*b"KAMES",
	*b"KAMIK",*b"KAMIS",*b"KAMME",*b"KANAE",*b"KANAS",*b"KANDY",*b"KANEH",*b"KANES",*b"KANGA",*b"KANGS",
	*b"KANJI",*b"KANTS",*b"KANZU",*b"KAONS",*b"KAPAS",*b"KAPHS",*b"KAPOK",*b"KAPOW",*b"KAPPA",*b"KAPUS",
	*b"KAPUT",*b"KARAS",*b"KARAT",*b"KARKS",*b"KARMA",*b"KARNS",*b"KAROO",*b"KAROS",*b"KARRI",*b"KARST",
	*b"KARSY",*b"KARTS",*b"KARZY",*b"KASHA",*b"KASME",*b"KATAL",*b"KATAS",*b"KATIS",*b"KATTI",*b"KAUGH",
	*b"KAURI",*b"KAURU",*b"KAURY",*b"KAVAL",*b"KAVAS",*b"KAWAS",*b"KAWAU",*b"KAWED",*b"KAYAK",*b"KAYLE",
	*b"KAYOS",*b"KAZIS",*b"KAZOO",*b"KBARS",*b"KEBAB",*b"KEBAR",*b"KEBOB",*b"KECKS",*b"KEDGE",*b"KEDGY",
	*b"KEECH",*b"KEEFS",*b"KEEKS",*b"KEELS",*b"KEEMA",*b"KEENO",*b"KEENS",*b"KEEPS",*b"KEETS",*b"KEEVE",
	*b"KEFIR",*b"KEHUA",*b"KEIRS",*b"KELEP",*b"KELIM",*b"KELLS",*b"KELLY",*b"KELPS",*b"KELPY",*b"KELTS",
	*b"KELTY",*b"KEMBO",*b"KEMBS",*b"KEMPS",*b"KEMPT",*b"KEMPY",*b"KENAF",*b"KENCH",*b"KENDO",*b"KENOS",
	*b"KENTE",*b"KENTS",*b"KEPIS",*b"KERBS",*b"KEREL",*b"KERFS",*b"KERKY",*b"KERMA",*b"KERNE",*b"KERNS",
	*b"KEROS",*b"KERRY",*b"KERVE",*b"KESAR",*b"KESTS",*b"KETAS",*b"KETCH",*b"KETES",*b"KETOL",*b"KEVEL",
	*b"KEVIL",*b"KEXES",*b"KEYED",*b"KEYER",*b"KHADI",*b"KHAFS",*b"KHAKI",*b"KHANS",*b"KHAPH",*b"KHATS",
	*b"KHAYA",*b"KHAZI",*b"KHEDA",*b"KHETH",*b"KHETS",*b"KHOJA",*b"KHORS",*b"KHOUM",*b"KHUDS",*b"KIAAT",
	*b"KIACK",*b"KIANG",*b"KIBBE",*b"KIBBI",*b"KIBEI",*b"KIBES",*b"KIBLA",*b"KICKS",*b"KICKY",*b"KIDDO",
	*b"KIDDY",*b"KIDEL",*b"KIDGE",*b"KIEFS",*b"KIERS",*b"KIEVE",*b"KIEVS",*b"KIGHT",*b"KIKES",*b"KIKOI",
	*b"KILEY",*b"KILIM",*b"KILLS",*b"KILNS",*b"KILOS",*b"KILPS",*b"KILTS",*b"KILTY",*b"KIMBO",*b"KINAS",
	*b"KINDA",*b"KINDS",*b"KINDY",*b"KINES",*b"KINGS",*b"KININ",*b"KINKS",*b"KINKY",*b"KINOS",*b"KIORE",
	*b"KIOSK",*b"KIPES",*b"KIPPA",*b"KIPPS",*b"KIRBY",*b"KIRKS",*b"KIRNS",*b"KIRRI",*b"KISAN",*b"KISSY",
	*b"KISTS",*b"KITED",*b"KITER",*b"KITES",*b"KITHE",*b"KITHS",*b"KITTY",*b"KITUL",*b"KIVAS",*b"KIWIS",
	*b"KLANG",*b"KLAPS",*b"KLETT",*b"KLICK",*b"KLIEG",*b"KLIKS",*b"KLONG",*b"KLOOF",*b"KLUGE",*b"KLUTZ",
	*b"KNACK",*b"KNAGS",*b"KNAPS",*b"KNARL",*b"KNARS",*b"KNAUR",*b"KNAVE",*b"KNAWE",*b"KNEAD",*b"KNEED",
	*b"KNEEL",*b"KNEES",*b"KNELL",*b"KNELT",*b"KNIFE",*b"KNISH",*b"KNITS",*b"KNIVE",*b"KNOBS",*b"KNOCK",
	*b"KNOLL",*b"KNOPS",*b"KNOSP",*b"KNOTS",*b"KNOUT",*b"KNOWE",*b"KNOWN",*b"KNOWS",*b"KNUBS",*b"KNURL",
	*b"KNURR",*b"KNURS",*b"KNUTS",*b"KOALA",*b"KOANS",*b"KOAPS",*b"KOBAN",*b"KOBOS",*b"KOELS",*b"KOFFS",
	*b"KOFTA",*b"KOGAL",*b"KOHAS",*b"KOHEN",*b"KOHLS",*b"KOINE",*b"KOJIS",*b"KOKAM",*b"KOKAS",*b"KOKER",
	*b"KOKRA",*b"KOKUM",*b"KOLAS",*b"KOLOS",*b"KOMBU",*b"KONBU",*b"KONDO",*b"KONKS",*b"KOOKS",*b"KOOKY",
	*b"KOORI",*b"KOPEK",*b"KOPHS",*b"KOPJE",*b"KOPPA",*b"KORAI",*b"KORAS",*b"KORAT",*b"KORES",*b"KORMA",
	*b"KOROS",*b"KORUN",*b"KORUS",*b"KOSES",*b"KOTCH",*b"KOTOS",*b"KOTOW",*b"KOURA",*b"KRAAL",*b"KRABS",
	*b"KRAFT",*b"KRAIS",*b"KRAIT",*b"KRANG",*b"KRANS",*b"KRANZ",*b"KRAUT",*b"KRAYS",*b"KREEP",*b"KRENG",
	*b"KREWE",*b"KRILL",*b"KRONA",*b"KRONE",*b"KROON",*b"KRUBI",*b"KRUNK",*b"KSARS",*b"KUBIE",*b"KUDOS",
	*b"KUDUS",*b"KUDZU",*b"KUFIS",*b"KUGEL",*b"KUIAS",*b"KUKRI",*b"KUKUS",*b"KULAK",*b"KULAN",*b"KULAS",
	*b"KULFI",*b"KUMIS",*b"KUMYS",*b"KURIS",*b"KURRE",*b"KURTA",*b"KURUS",*b"KUSSO",*b"KUTAS",*b"KUTCH",
	*b"KUTIS",*b"KUTUS",*b"KUZUS",*b"KVASS",*b"KVELL",*b"KWELA",*b"KYACK",*b"KYAKS",*b"KYANG",*b"KYARS",
	*b"KYATS",*b"KYBOS",*b"KYDST",*b"KYLES",*b"KYLIE",*b"KYLIN",*b"KYLIX",*b"KYLOE",*b"KYNDE",*b"KYNDS",
	*b"KYPES",*b"KYRIE",*b"KYTES",*b"KYTHE",*b"LAARI",*b"LABDA",*b"LABEL",*b"LABIA",*b"LABIS",*b"LABOR",
	*b"LABRA",*b"LACED",*b"LACER",*b"LACES",*b"LACET",*b"LACEY",*b"LACKS",*b"LADDY",*b"LADED",*b"LADEN",
	*b"LADER",*b"LADES",*b"LADLE",*b"LAERS",*b"LAEVO",*b"LAGAN",*b"LAGER",*b"LAHAL",*b"LAHAR",*b"LAICH",
	*b"LAICS",*b"LAIDS",*b"LAIGH",*b"LAIKA",*b"LAIKS",*b"LAIRD",*b"LAIRS",*b"LAIRY",*b"LAITH",*b"LAITY",
	*b"LAKED",*b"LAKER",*b"LAKES",*b"LAKHS",*b"LAKIN",*b"LAKSA",*b"LALDY",*b"LALLS",*b"LAMAS",*b"LAMBS",
	*b"LAMBY",*b"LAMED",*b"LAMER",*b"LAMES",*b"LAMIA",*b"LAMMY",*b"LAMPS",*b"LANAI",*b"LANAS",*b"LANCE",
	*b"LANCH",*b"LANDE",*b"LANDS",*b"LANES",*b"LANKS",*b"LANKY",*b"LANTS",*b"LAPEL",*b"LAPIN",*b"LAPIS",
	*b"LAPJE",*b"LAPSE",*b"LARCH",*b"LARDS",*b"LARDY",*b"LAREE",*b"LARES",*b"LARGE",*b"LARGO",*b"LARIS",
	*b"LARKS",*b"LARKY",*b"LARNS",*b"LARNT",*b"LARUM",*b"LARVA",*b"LASED",*b"LASER",*b"LASES",*b"LASSI",
	*b"LASSO",*b"LASSU",*b"LASSY",*b"LASTS",*b"LATAH",*b"LATCH",*b"LATED",*b"LATEN",*b"LATER",*b"LATEX",
	*b"LATHE",*b"LATHI",*b"LATHS",*b"LATHY",*b"LATKE",*b"LATTE",*b"LATUS",*b"LAUAN",*b"LAUCH",*b"LAUDS",
	*b"LAUFS",*b"LAUGH",*b"LAUND",*b"LAURA",*b"LAVAL",*b"LAVAS",*b"LAVED",*b"LAVER",*b"LAVES",*b"LAVRA",
	*b"LAVVY",*b"LAWED",*b"LAWER",*b"LAWIN",*b"LAWKS",*b"LAWNS",*b"LAWNY",*b"LAXED",*b"LAXER",*b"LAXES",
	*b"LAXLY",*b"LAYED",*b"LAYER",*b"LAYIN",*b"LAYUP",*b"LAZAR",*b"LAZED",*b"LAZES",*b"LAZOS",*b"LAZZI",
	*b"LAZZO",*b"LEACH",*b"LEADS",*b"LEADY",*b"LEAFS",*b"LEAFY",*b"LEAKS",*b"LEAKY",*b"LEAMS",*b"LEANS",
	*b"LEANT",*b"LEANY",*b"LEAPS",*b"LEAPT",*b"LEARE",*b"LEARN",*b"LEARS",*b"LEARY",*b"LEASE",*b"LEASH",
	*b"LEAST",*b"LEATS",*b"LEAVE",*b"LEAVY",*b"LEAZE",*b"LEBEN",*b"LECCY",*b"LEDES",*b"LEDGE",*b"LEDGY",
	*b"LEDUM",*b"LEEAR",*b"LEECH",*b"LEEKS",*b"LEEPS",*b"LEERS",*b"LEERY",*b"LEESE",*b"LEETS",*b"LEEZE",
	*b"LEFTE",*b"LEFTS",*b"LEFTY",*b"LEGAL",*b"LEGER",*b"LEGES",*b"LEGGE",*b"LEGGO",*b"LEGGY",*b"LEGIT",
	*b"LEHRS",*b"LEHUA",*b"LEIRS",*b"LEISH",*b"LEMAN",*b"LEMED",*b"LEMEL",*b"LEMES",*b"LEMMA",*b"LEMME",
	*b"LEMON",*b"LEMUR",*b"LENDS",*b"LENES",*b"LENGS",*b"LENIS",*b"LENOS",*b"LENSE",*b"LENTI",*b"LENTO",
	*b"LEONE",*b"LEPER",*b"LEPID",*b"LEPRA",*b"LEPTA",*b"LERED",*b"LERES",*b"LERPS",*b"LESBO",*b"LESES",
	*b"LESTS",*b"LETCH",*b"LETHE",*b"LETUP",*b"LEUCH",*b"LEUCO",*b"LEUDS",*b"LEUGH",*b"LEVAS",*b"LEVEE",
	*b"LEVEL",*b"LEVER",*b"LEVES",*b"LEVIN",*b"LEVIS",*b"LEWIS",*b"LEXES",*b"LEXIS",*b"LEZES",*b"LEZZA",
	*b"LEZZY",*b"LIANA",*b"LIANE",*b"LIANG",*b"LIARD",*b"LIARS",*b"LIART",*b"LIBEL",*b"LIBER",*b"LIBRA",
	*b"LIBRI",*b"LICHI",*b"LICHT",*b"LICIT",*b"LICKS",*b"LIDAR",*b"LIDOS",*b"LIEFS",*b"LIEGE",*b"LIENS",
	*b"LIERS",*b"LIEUS",*b"LIEVE",*b"LIFER",*b"LIFES",*b"LIFTS",*b"LIGAN",*b"LIGER",*b"LIGGE",*b"LIGHT",
	*b"LIGNE",*b"LIKED",*b"LIKEN",*b"LIKER",*b"LIKES",*b"LIKIN",*b"LILAC",*b"LILLS",*b"LILOS",*b"LILTS",
	*b"LIMAN",*b"LIMAS",*b"LIMAX",*b"LIMBA",*b"LIMBI",*b"LIMBO",*b"LIMBS",*b"LIMBY",*b"LIMED",*b"LIMEN",
	*b"LIMES",*b"LIMEY",*b"LIMIT",*b"LIMMA",*b"LIMNS",*b"LIMOS",*b"LIMPA",*b"LIMPS",*b"LINAC",*b"LINCH",
	*b"LINDS",*b"LINDY",*b"LINED",*b"LINEN",*b"LINER",*b"LINES",*b"LINEY",*b"LINGA",*b"LINGO",*b"LINGS",
	*b"LINGY",*b"LININ",*b"LINKS",*b"LINKY",*b"LINNS",*b"LINNY",*b"LINOS",*b"LINTS",*b"LINTY",*b"LINUM",
	*b"LINUX",*b"LIONS",*b"LIPAS",*b"LIPES",*b"LIPID",*b"LIPIN",*b"LIPOS",*b"LIPPY",*b"LIRAS",*b"LIRKS",
	*b"LIROT",*b"LISKS",*b"LISLE",*b"LISPS",*b"LISTS",*b"LITAI",*b"LITAS",*b"LITED",*b"LITER",*b"LITES",
	*b"LITHE",*b"LITHO",*b"LITHS",*b"LITRE",*b"LIVED",*b"LIVEN",*b"LIVER",*b"LIVES",*b"LIVID",*b"LIVOR",
	*b"LIVRE",*b"LLAMA",*b"LLANO",*b"LOACH",*b"LOADS",*b"LOAFS",*b"LOAMS",*b"LOAMY",*b"LOANS",*b"LOAST",
	*b"LOATH",*b"LOAVE",*b"LOBAR",*b"LOBBY",*b"LOBED",*b"LOBES",*b"LOBOS",*b"LOBUS",*b"LOCAL",*b"LOCHE",
	*b"LOCHS",*b"LOCIE",*b"LOCIS",*b"LOCKS",*b"LOCOS",*b"LOCUM",*b"LOCUS",*b"LODEN",*b"LODES",*b"LODGE",
	*b"LOESS",*b"LOFTS",*b"LOFTY",*b"LOGAN",*b"LOGES",*b"LOGGY",*b"LOGIA",*b"LOGIC",*b"LOGIE",*b"LOGIN",
	*b"LOGOI",*b"LOGON",*b"LOGOS",*b"LOHAN",*b"LOIDS",*b"LOINS",*b"LOIPE",*b"LOIRS",*b"LOKES",*b"LOLLS",
	*b"LOLLY",*b"LOLOG",*b"LOMAS",*b"LOMED",*b"LOMES",*b"LONER",*b"LONGA",*b"LONGE",*b"LONGS",*b"LOOBY",
	*b"LOOED",*b"LOOEY",*b"LOOFA",*b"LOOFS",*b"LOOIE",*b"LOOKS",*b"LOOKY",*b"LOOMS",*b"LOONS",*b"LOONY",
	*b"LOOPS",*b"LOOPY",*b"LOORD",*b"LOOSE",*b"LOOTS",*b"LOPED",*b"LOPER",*b"LOPES",*b"LOPPY",*b"LORAL",
	*b"LORAN",*b"LORDS",*b"LORDY",*b"LOREL",*b"LORES",*b"LORIC",*b"LORIS",*b"LORRY",*b"LOSED",*b"LOSEL",
	*b"LOSEN",*b"LOSER",*b"LOSES",*b"LOSSY",*b"LOTAH",*b"LOTAS",*b"LOTES",*b"LOTIC",*b"LOTOS",*b"LOTSA",
	*b"LOTTA",*b"LOTTE",*b"LOTTO",*b"LOTUS",*b"LOUED",*b"LOUGH",*b"LOUIE",*b"LOUIS",*b"LOUMA",*b"LOUND",
	*b"LOUNS",*b"LOUPE",*b"LOUPS",*b"LOURE",*b"LOURS",*b"LOURY",*b"LOUSE",*b"LOUSY",*b"LOUTS",*b"LOVAT",
	*b"LOVED",*b"LOVER",*b"LOVES",*b"LOVEY",*b"LOVIE",*b"LOWAN",*b"LOWED",*b"LOWER",*b"LOWES",*b"LOWLY",
	*b"LOWND",*b"LOWNE",*b"LOWNS",*b"LOWPS",*b"LOWRY",*b"LOWSE",*b"LOWTS",*b"LOXED",*b"LOXES",*b"LOYAL",
	*b"LOZEN",*b"LUACH",*b"LUAUS",*b"LUBED",*b"LUBES",*b"LUBRA",*b"LUCES",*b"LUCID",*b"LUCKS",*b"LUCKY",
	*b"LUCRE",*b"LUDES",*b"LUDIC",*b"LUDOS",*b"LUFFA",*b"LUFFS",*b"LUGED",*b"LUGER",*b"LUGES",*b"LULLS",
	*b"LULUS",*b"LUMAS",*b"LUMBI",*b"LUMEN",*b"LUMME",*b"LUMMY",*b"LUMPS",*b"LUMPY",*b"LUNAR",*b"LUNAS",
	*b"LUNCH",*b"LUNES",*b"LUNET",*b"LUNGE",*b"LUNGI",*b"LUNGS",*b"LUNKS",*b"LUNTS",*b"LUPIN",*b"LUPUS",
	*b"LURCH",*b"LURED",*b"LURER",*b"LURES",*b"LUREX",*b"LURGI",*b"LURGY",*b"LURID",*b"LURKS",*b"LURRY",
	*b"LURVE",*b"LUSER",*b"LUSHY",*b"LUSKS",*b"LUSTS",*b"LUSTY",*b"LUSUS",*b"LUTEA",*b"LUTED",*b"LUTER",
	*b"LUTES",*b"LUVVY",*b"LUXED",*b"LUXER",*b"LUXES",*b"LWEIS",*b"LYAMS",*b"LYARD",*b"LYART",*b"LYASE",
	*b"LYCEA",*b"LYCEE",*b"LYCRA",*b"LYING",*b"LYMES",*b"LYMPH",*b"LYNCH",*b"LYNES",*b"LYRES",*b"LYRIC",
	*b"LYSED",*b"LYSES",*b"LYSIN",*b"LYSIS",*b"LYSOL",*b"LYSSA",*b"LYTED",*b"LYTES",*b"LYTHE",*b"LYTIC",
	*b"LYTTA",*b"MAAED",*b"MAARE",*b"MAARS",*b"MABES",*b"MACAS",*b"MACAW",*b"MACED",*b"MACER",*b"MACES",
	*b"MACHE",*b"MACHI",*b"MACHO",*b"MACHS",*b"MACKS",*b"MACLE",*b"MACON",*b"MACRO",*b"MADAM",*b"MADGE",
	*b"MADID",*b"MADLY",*b"MADRE",*b"MAERL",*b"MAFIA",*b"MAFIC",*b"MAGES",*b"MAGGS",*b"MAGIC",*b"MAGMA",
	*b"MAGOT",*b"MAGUS",*b"MAHOE",*b"MAHUA",*b"MAHWA",*b"MAIDS",*b"MAIKO",*b"MAIKS",*b"MAILE",*b"MAILL",
	*b"MAILS",*b"MAIMS",*b"MAINS",*b"MAIRE",*b"MAIRS",*b"MAISE",*b"MAIST",*b"MAIZE",*b"MAJOR",*b"MAKAR",
	*b"MAKER",*b"MAKES",*b"MAKIS",*b"MAKOS",*b"MALAM",*b"MALAR",*b"MALAS",*b"MALAX",*b"MALES",*b"MALIC",
	*b"MALIK",*b"MALIS",*b"MALLS",*b"MALMS",*b"MALMY",*b"MALTS",*b"MALTY",*b"MALUS",*b"MALVA",*b"MALWA",
	*b"MAMAS",*b"MAMBA",*b"MAMBO",*b"MAMEE",*b"MAMEY",*b"MAMIE",*b"MAMMA",*b"MAMMY",*b"MANAS",*b"MANAT",
	*b"MANDI",*b"MANEB",*b"MANED",*b"MANEH",*b"MANES",*b"MANET",*b"MANGA",*b"MANGE",*b"MANGO",*b"MANGS",
	*b"MANGY",*b"MANIA",*b"MANIC",*b"MANIS",*b"MANKY",*b"MANLY",*b"MANNA",*b"MANOR",*b"MANOS",*b"MANSE",
	*b"MANTA",*b"MANTO",*b"MANTY",*b"MANUL",*b"MANUS",*b"MAPAU",*b"MAPLE",*b"MAQUI",*b"MARAE",*b"MARAH",
	*b"MARAS",*b"MARCH",*b"MARCS",*b"MARDY",*b"MARES",*b"MARGE",*b"MARGS",*b"MARIA",*b"MARID",*b"MARKA",
	*b"MARKS",*b"MARLE",*b"MARLS",*b"MARLY",*b"MARMS",*b"MARON",*b"MAROR",*b"MARRA",*b"MARRI",*b"MARRY",
	*b"MARSE",*b"MARSH",*b"MARTS",*b"MARVY",*b"MASAS",*b"MASED",*b"MASER",*b"MASES",*b"MASHY",*b"MASKS",
	*b"MASON",*b"MASSA",*b"MASSE",*b"MASSY",*b"MASTS",*b"MASTY",*b"MASUS",*b"MATAI",*b"MATCH",*b"MATED",
	*b"MATER",*b"MATES",*b"MATEY",*b"MATHS",*b"MATIN",*b"MATLO",*b"MATTE",*b"MATTS",*b"MATZA",*b"MATZO",
	*b"MAUBY",*b"MAUDS",*b"MAULS",*b"MAUND",*b"MAURI",*b"MAUSY",*b"MAUTS",*b"MAUVE",*b"MAUZY",*b"MAVEN",
	*b"MAVIE",*b"MAVIN",*b"MAVIS",*b"MAWED",*b"MAWKS",*b"MAWKY",*b"MAWNS",*b"MAWRS",*b"MAXED",*b"MAXES",
	*b"MAXIM",*b"MAXIS",*b"MAYAN",*b"MAYAS",*b"MAYBE",*b"MAYED",*b"MAYOR",*b"MAYOS",*b"MAYST",*b"MAZED",
	*b"MAZER",*b"MAZES",*b"MAZEY",*b"MAZUT",*b"MBIRA",*b"MEADS",*b"MEALS",*b"MEALY",*b"MEANE",*b"MEANS",
	*b"MEANT",*b"MEANY",*b"MEARE",*b"MEASE",*b"MEATH",*b"MEATS",*b"MEATY",*b"MEBOS",*b"MECCA",*b"MECHS",
	*b"MECKS",*b"MEDAL",*b"MEDIA",*b"MEDIC",*b"MEDII",*b"MEDLE",*b"MEEDS",*b"MEERS",*b"MEETS",*b"MEFFS",
	*b"MEINS",*b"MEINT",*b"MEINY",*b"MEITH",*b"MEKKA",*b"MELAS",*b"MELBA",*b"MELDS",*b"MELEE",*b"MELIC",
	*b"MELIK",*b"MELLS",*b"MELON",*b"MELTS",*b"MELTY",*b"MEMES",*b"MEMOS",*b"MENAD",*b"MENDS",*b"MENED",
	*b"MENES",*b"MENGE",*b"MENGS",*b"MENSA",*b"MENSE",*b"MENSH",*b"MENTA",*b"MENTO",*b"MENUS",*b"MEOUS",
	*b"MEOWS",*b"MERCH",*b"MERCS",*b"MERCY",*b"MERDE",*b"MERED",*b"MEREL",*b"MERER",*b"MERES",*b"MERGE",
	*b"MERIL",*b"MERIS",*b"MERIT",*b"MERKS",*b"MERLE",*b"MERLS",*b"MERRY",*b"MERSE",*b"MESAL",*b"MESAS",
	*b"MESEL",*b"MESES",*b"MESHY",*b"MESIC",*b"MESNE",*b"MESON",*b"MESSY",*b"MESTO",*b"METAL",*b"METED",
	*b"METER",*b"METES",*b"METHO",*b"METHS",*b"METIC",*b"METIF",*b"METIS",*b"METOL",*b"METRE",*b"METRO",
	*b"MEUSE",*b"MEVED",*b"MEVES",*b"MEWED",*b"MEWLS",*b"MEYNT",*b"MEZES",*b"MEZZE",*b"MEZZO",*b"MHORR",
	*b"MIAOU",*b"MIAOW",*b"MIASM",*b"MIAUL",*b"MICAS",*b"MICHE",*b"MICHT",*b"MICKS",*b"MICKY",*b"MICOS",
	*b"MICRA",*b"MICRO",*b"MIDDY",*b"MIDGE",*b"MIDGY",*b"MIDIS",*b"MIDST",*b"MIENS",*b"MIEVE",*b"MIFFS",
	*b"MIFFY",*b"MIFTY",*b"MIGGS",*b"MIGHT",*b"MIHAS",*b"MIHIS",*b"MIKED",*b"MIKES",*b"MIKRA",*b"MIKVA",
	*b"MILCH",*b"MILDS",*b"MILER",*b"MILES",*b"MILFS",*b"MILIA",*b"MILKO",*b"MILKS",*b"MILKY",*b"MILLE",
	*b"MILLS",*b"MILOR",*b"MILOS",*b"MILPA",*b"MILTS",*b"MILTY",*b"MILTZ",*b"MIMED",*b"MIMEO",*b"MIMER",
	*b"MIMES",*b"MIMIC",*b"MIMSY",*b"MINAE",*b"MINAR",*b"MINAS",*b"MINCE",*b"MINCY",*b"MINDS",*b"MINED",
	*b"MINER",*b"MINES",*b"MINGE",*b"MINGS",*b"MINGY",*b"MINIM",*b"MINIS",*b"MINKE",*b"MINKS",*b"MINNY",
	*b"MINOR",*b"MINOS",*b"MINTS",*b"MINTY",*b"MINUS",*b"MIRED",*b"MIRES",*b"MIREX",*b"MIRID",*b"MIRIN",
	*b"MIRKS",*b"MIRKY",*b"MIRLY",*b"MIROS",*b"MIRTH",*b"MIRVS",*b"MIRZA",*b"MISCH",*b"MISDO",*b"MISER",
	*b"MISES",*b"MISGO",*b"MISOS",*b"MISSA",*b"MISSY",*b"MISTS",*b"MISTY",*b"MITCH",*b"MITER",*b"MITES",
	*b"MITIS",*b"MITRE",*b"MITTS",*b"MIXED",*b"MIXEN",*b"MIXER",*b"MIXES",*b"MIXTE",*b"MIXUP",*b"MIZEN",
	*b"MIZZY",*b"MNEME",*b"MOANS",*b"MOATS",*b"MOBBY",*b"MOBES",*b"MOBEY",*b"MOBIE",*b"MOBLE",*b"MOCHA",
	*b"MOCHI",*b"MOCHS",*b"MOCHY",*b"MOCKS",*b"MODAL",*b"MODEL",*b"MODEM",*b"MODER",*b"MODES",*b"MODGE",
	*b"MODII",*b"MODUS",*b"MOERS",*b"MOFOS",*b"MOGGY",*b"MOGUL",*b"MOHEL",*b"MOHOS",*b"MOHRS",*b"MOHUA",
	*b"MOHUR",*b"MOILE",*b"MOILS",*b"MOIRA",*b"MOIRE",*b"MOIST",*b"MOITS",*b"MOJOS",*b"MOKES",*b"MOKIS",
	*b"MOKOS",*b"MOLAL",*b"MOLAR",*b"MOLAS",*b"MOLDS",*b"MOLDY",*b"MOLED",*b"MOLES",*b"MOLLA",*b"MOLLS",
	*b"MOLLY",*b"MOLTO",*b"MOLTS",*b"MOLYS",*b"MOMES",*b"MOMMA",*b"MOMMY",*b"MOMUS",*b"MONAD",*b"MONAL",
	*b"MONAS",*b"MONDE",*b"MONDO",*b"MONER",*b"MONEY",*b"MONGO",*b"MONGS",*b"MONIC",*b"MONIE",*b"MONKS",
	*b"MONOS",*b"MONTE",*b"MONTH",*b"MONTY",*b"MOOBS",*b"MOOCH",*b"MOODS",*b"MOODY",*b"MOOED",*b"MOOKS",
	*b"MOOLA",*b"MOOLI",*b"MOOLS",*b"MOOLY",*b"MOONG",*b"MOONS",*b"MOONY",*b"MOOPS",*b"MOORS",*b"MOORY",
	*b"MOOSE",*b"MOOTS",*b"MOOVE",*b"MOPED",*b"MOPER",*b"MOPES",*b"MOPEY",*b"MOPPY",*b"MOPSY",*b"MOPUS",
	*b"MORAE",*b"MORAL",*b"MORAS",*b"MORAT",*b"MORAY",*b"MOREL",*b"MORES",*b"MORIA",*b"MORNE",*b"MORNS",
	*b"MORON",*b"MORPH",*b"MORRA",*b"MORRO",*b"MORSE",*b"MORTS",*b"MOSED",*b"MOSES",*b"MOSEY",*b"MOSKS",
	*b"MOSSO",*b"MOSSY",*b"MOSTE",*b"MOSTS",*b"MOTED",*b"MOTEL",*b"MOTEN",*b"MOTES",*b"MOTET",*b"MOTEY",
	*b"MOTHS",*b"MOTHY",*b"MOTIF",*b"MOTIS",*b"MOTOR",*b"MOTTE",*b"MOTTO",*b"MOTTS",*b"MOTTY",*b"MOTUS",
	*b"MOTZA",*b"MOUCH",*b"MOUES",*b"MOULD",*b"MOULS",*b"MOULT",*b"MOUND",*b"MOUNT",*b"MOUPS",*b"MOURN",
	*b"MOUSE",*b"MOUST",*b"MOUSY",*b"MOUTH",*b"MOVED",*b"MOVER",*b"MOVES",*b"MOVIE",*b"MOWAS",*b"MOWED",
	*b"MOWER",*b"MOWRA",*b"MOXAS",*b"MOXIE",*b"MOYAS",*b"MOYLE",*b"MOYLS",*b"MOZED",*b"MOZES",*b"MOZOS",
	*b"MPRET",*b"MUCHO",*b"MUCIC",*b"MUCID",*b"MUCIN",*b"MUCKS",*b"MUCKY",*b"MUCOR",*b"MUCRO",*b"MUCUS",
	*b"MUDDY",*b"MUDGE",*b"MUDIR",*b"MUDRA",*b"MUFFS",*b"MUFTI",*b"MUGGA",*b"MUGGS",*b"MUGGY",*b"MUHLY",
	*b"MUIDS",*b"MUILS",*b"MUIRS",*b"MUIST",*b"MUJIK",*b"MULCH",*b"MULCT",*b"MULED",*b"MULES",*b"MULEY",
	*b"MULGA",*b"MULIE",*b"MULLA",*b"MULLS",*b"MULSE",*b"MULSH",*b"MUMMS",*b"MUMMY",*b"MUMPS",*b"MUMSY",
	*b"MUMUS",*b"MUNCH",*b"MUNGA",*b"MUNGE",*b"MUNGO",*b"MUNGS",*b"MUNIS",*b"MUNTS",*b"MUNTU",*b"MUONS",
	*b"MURAL",*b"MURAS",*b"MURED",*b"MURES",*b"MUREX",*b"MURID",*b"MURKS",*b"MURKY",*b"MURLS",*b"MURLY",
	*b"MURRA",*b"MURRE",*b"MURRI",*b"MURRS",*b"MURRY",*b"MURTI",*b"MURVA",*b"MUSAR",*b"MUSCA",*b"MUSED",
	*b"MUSER",*b"MUSES",*b"MUSET",*b"MUSHA",*b"MUSHY",*b"MUSIC",*b"MUSIT",*b"MUSKS",*b"MUSKY",*b"MUSOS",
	*b"MUSSE",*b"MUSSY",*b"MUSTH",*b"MUSTS",*b"MUSTY",*b"MUTCH",*b"MUTED",*b"MUTER",*b"MUTES",*b"MUTHA",
	*b"MUTIS",*b"MUTON",*b"MUTTS",*b"MUXED",*b"MUXES",*b"MUZAK",*b"MUZZY",*b"MVULE",*b"MYALL",*b"MYLAR",
	*b"MYNAH",*b"MYNAS",*b"MYOID",*b"MYOMA",*b"MYOPE",*b"MYOPS",*b"MYOPY",*b"MYRRH",*b"MYSID",*b"MYTHI",
	*b"MYTHS",*b"MYTHY",*b"MYXOS",*b"MZEES",*b"NAAMS",*b"NAANS",*b"NABES",*b"NABIS",*b"NABKS",*b"NABLA",
	*b"NABOB",*b"NACHE",*b"NACHO",*b"NACRE",*b"NADAS",*b"NADIR",*b"NAEVE",*b"NAEVI",*b"NAFFS",*b"NAGAS",
	*b"NAGGY",*b"NAGOR",*b"NAHAL",*b"NAIAD",*b"NAIFS",*b"NAIKS",*b"NAILS",*b"NAIRA",*b"NAIRU",*b"NAIVE",
	*b"NAKED",*b"NAKER",*b"NAKFA",*b"NALAS",*b"NALED",*b"NALLA",*b"NAMED",*b"NAMER",*b"NAMES",*b"NAMMA",
	*b"NAMUS",*b"NANAS",*b"NANCE",*b"NANCY",*b"NANDU",*b"NANNA",*b"NANNY",*b"NANOS",*b"NANUA",*b"NAPAS",
	*b"NAPED",*b"NAPES",*b"NAPOO",*b"NAPPA",*b"NAPPE",*b"NAPPY",*b"NARAS",*b"NARCO",*b"NARCS",*b"NARDS",
	*b"NARES",*b"NARIC",*b"NARIS",*b"NARKS",*b"NARKY",*b"NARRE",*b"NASAL",*b"NASHI",*b"NASTY",*b"NATAL",
	*b"NATCH",*b"NATES",*b"NATIS",*b"NATTY",*b"NAUCH",*b"NAUNT",*b"NAVAL",*b"NAVAR",*b"NAVEL",*b"NAVES",
	*b"NAVEW",*b"NAVVY",*b"NAWAB",*b"NAZES",*b"NAZIR",*b"NAZIS",*b"NDUJA",*b"NEAFE",*b"NEALS",*b"NEAPS",
	*b"NEARS",*b"NEATH",*b"NEATS",*b"NEBEK",*b"NEBEL",*b"NECKS",*b"NEDDY",*b"NEEDS",*b"NEEDY",*b"NEELD",
	*b"NEELE",*b"NEEMB",*b"NEEMS",*b"NEEPS",*b"NEESE",*b"NEEZE",*b"NEGRO",*b"NEGUS",*b"NEIFS",*b"NEIGH",
	*b"NEIST",*b"NEIVE",*b"NELIS",*b"NELLY",*b"NEMAS",*b"NEMNS",*b"NEMPT",*b"NENES",*b"NEONS",*b"NEPER",
	*b"NEPIT",*b"NERAL",*b"NERDS",*b"NERDY",*b"NERKA",*b"NERKS",*b"NEROL",*b"NERTS",*b"NERTZ",*b"NERVE",
	*b"NERVY",*b"NESTS",*b"NETES",*b"NETOP",*b"NETTS",*b"NETTY",*b"NEUKS",*b"NEUME",*b"NEUMS",*b"NEVEL",
	*b"NEVER",*b"NEVES",*b"NEVUS",*b"NEWBS",*b"NEWED",*b"NEWEL",*b"NEWER",*b"NEWIE",*b"NEWLY",*b"NEWSY",
	*b"NEWTS",*b"NEXTS",*b"NEXUS",*b"NGAIO",*b"NGANA",*b"NGATI",*b"NGOMA",*b"NGWEE",*b"NICAD",*b"NICER",
	*b"NICHE",*b"NICHT",*b"NICKS",*b"NICOL",*b"NIDAL",*b"NIDED",*b"NIDES",*b"NIDOR",*b"NIDUS",*b"NIECE",
	*b"NIEFS",*b"NIEVE",*b"NIFES",*b"NIFFS",*b"NIFFY",*b"NIFTY",*b"NIGER",*b"NIGHS",*b"NIGHT",*b"NIHIL",
	*b"NIKAB",*b"NIKAH",*b"NIKAU",*b"NILLS",*b"NIMBI",*b"NIMBS",*b"NIMPS",*b"NINER",*b"NINES",*b"NINJA",
	*b"NINNY",*b"NINON",*b"NINTH",*b"NIPAS",*b"NIPPY",*b"NIQAB",*b"NIRLS",*b"NIRLY",*b"NISEI",*b"NISSE",
	*b"NISUS",*b"NITER",*b"NITES",*b"NITID",*b"NITON",*b"NITRE",*b"NITRO",*b"NITRY",*b"NITTY",*b"NIVAL",
	*b"NIXED",*b"NIXER",*b"NIXES",*b"NIXIE",*b"NIZAM",*b"NKOSI",*b"NOAHS",*b"NOBBY",*b"NOBLE",*b"NOBLY",
	*b"NOCKS",*b"NODAL",*b"NODDY",*b"NODES",*b"NODUS",*b"NOELS",*b"NOGGS",*b"NOHOW",*b"NOILS",*b"NOILY",
	*b"NOINT",*b"NOIRS",*b"NOISE",*b"NOISY",*b"NOLES",*b"NOLLS",*b"NOLOS",*b"NOMAD",*b"NOMAS",*b"NOMEN",
	*b"NOMES",*b"NOMIC",*b"NOMOI",*b"NOMOS",*b"NONAS",*b"NONCE",*b"NONES",*b"NONET",*b"NONGS",*b"NONIS",
	*b"NONNY",*b"NONYL",*b"NOOBS",*b"NOOIT",*b"NOOKS",*b"NOOKY",*b"NOONS",*b"NOOPS",*b"NOOSE",*b"NOPAL",
	*b"NORIA",*b"NORIS",*b"NORKS",*b"NORMA",*b"NORMS",*b"NORTH",*b"NOSED",*b"NOSER",*b"NOSES",*b"NOSEY",
	*b"NOTAL",*b"NOTCH",*b"NOTED",*b"NOTER",*b"NOTES",*b"NOTUM",*b"NOULD",*b"NOULE",*b"NOULS",*b"NOUNS",
	*b"NOUNY",*b"NOUPS",*b"NOVAE",*b"NOVAS",*b"NOVEL",*b"NOVUM",*b"NOWAY",*b"NOWED",*b"NOWLS",*b"NOWTS",
	*b"NOWTY",*b"NOXAL",*b"NOXES",*b"NOYAU",*b"NOYED",*b"NOYES",*b"NUBBY",*b"NUBIA",*b"NUCHA",*b"NUDDY",
	*b"NUDER",*b"NUDES",*b"NUDGE",*b"NUDIE",*b"NUDZH",*b"NUFFS",*b"NUGAE",*b"NUKED",*b"NUKES",*b"NULLA",
	*b"NULLS",*b"NUMBS",*b"NUMEN",*b"NUMMY",*b"NUNNY",*b"NURDS",*b"NURDY",*b"NURLS",*b"NURRS",*b"NURSE",
	*b"NUTSO",*b"NUTSY",*b"NUTTY",*b"NYAFF",*b"NYALA",*b"NYING",*b"NYLON",*b"NYMPH",*b"NYSSA",*b"OAKED",
	*b"OAKEN",*b"OAKER",*b"OAKUM",*b"OARED",*b"OASES",*b"OASIS",*b"OASTS",*b"OATEN",*b"OATER",*b"OATHS",
	*b"OAVES",*b"OBANG",*b"OBEAH",*b"OBELI",*b"OBESE",*b"OBEYS",*b"OBIAS",*b"OBIED",*b"OBIIT",*b"OBITS",
	*b"OBJET",*b"OBOES",*b"OBOLE",*b"OBOLI",*b"OBOLS",*b"OCCAM",*b"OCCUR",*b"OCEAN",*b"OCHER",*b"OCHES",
	*b"OCHRE",*b"OCHRY",*b"OCKER",*b"OCREA",*b"OCTAD",*b"OCTAL",*b"OCTAN",*b"OCTAS",*b"OCTET",*b"OCTYL",
	*b"OCULI",*b"ODAHS",*b"ODALS",*b"ODDER",*b"ODDLY",*b"ODEON",*b"ODEUM",*b"ODISM",*b"ODIST",*b"ODIUM",
	*b"ODORS",*b"ODOUR",*b"ODYLE",*b"ODYLS",*b"OFAYS",*b"OFFAL",*b"OFFED",*b"OFFER",*b"OFFIE",*b"OFLAG",
	*b"OFTEN",*b"OFTER",*b"OGAMS",*b"OGEED",*b"OGEES",*b"OGGIN",*b"OGHAM",*b"OGIVE",*b"OGLED",*b"OGLER",
	*b"OGLES",*b"OGMIC",*b"OGRES",*b"OHIAS",*b"OHING",*b"OHMIC",*b"OHONE",*b"OIDIA",*b"OILED",*b"OILER",
	*b"OINKS",*b"OINTS",*b"OJIME",*b"OKAPI",*b"OKAYS",*b"OKEHS",*b"OKRAS",*b"OKTAS",*b"OLDEN",*b"OLDER",
	*b"OLDIE",*b"OLEIC",*b"OLEIN",*b"OLENT",*b"OLEOS",*b"OLEUM",*b"OLIOS",*b"OLIVE",*b"OLLAS",*b"OLLAV",
	*b"OLLER",*b"OLLIE",*b"OLOGY",*b"OLPAE",*b"OLPES",*b"OMASA",*b"OMBER",*b"OMBRE",*b"OMBUS",*b"OMEGA",
	*b"OMENS",*b"OMERS",*b"OMITS",*b"OMLAH",*b"OMOVS",*b"OMRAH",*b"ONCER",*b"ONCES",*b"ONCET",*b"ONCUS",
	*b"ONELY",*b"ONERS",*b"ONERY",*b"ONION",*b"ONIUM",*b"ONKUS",*b"ONLAY",*b"ONNED",*b"ONSET",*b"ONTIC",
	*b"OOBIT",*b"OOHED",*b"OOMPH",*b"OONTS",*b"OOPED",*b"OORIE",*b"OOSES",*b"OOTID",*b"OOZED",*b"OOZES",
	*b"OPAHS",*b"OPALS",*b"OPENS",*b"OPEPE",*b"OPERA",*b"OPINE",*b"OPING",*b"OPIUM",*b"OPPOS",*b"OPSIN",
	*b"OPTED",*b"OPTER",*b"OPTIC",*b"ORACH",*b"ORACY",*b"ORALS",*b"ORANG",*b"ORANT",*b"ORATE",*b"ORBED",
	*b"ORBIT",*b"ORCAS",*b"ORCIN",*b"ORDER",*b"ORDOS",*b"OREAD",*b"ORFES",*b"ORGAN",*b"ORGIA",*b"ORGIC",
	*b"ORGUE",*b"ORIBI",*b"ORIEL",*b"ORIXA",*b"ORLES",*b"ORLON",*b"ORLOP",*b"ORMER",*b"ORNIS",*b"ORPIN",
	*b"ORRIS",*b"ORTHO",*b"ORVAL",*b"ORZOS",*b"OSCAR",*b"OSHAC",*b"OSIER",*b"OSMIC",*b"OSMOL",*b"OSSIA",
	*b"OSTIA",*b"OTAKU",*b"OTARY",*b"OTHER",*b"OTTAR",*b"OTTER",*b"OTTOS",*b"OUBIT",*b"OUCHT",*b"OUENS",
	*b"OUGHT",*b"OUIJA",*b"OULKS",*b"OUMAS",*b"OUNCE",*b"OUNDY",*b"OUPAS",*b"OUPED",*b"OUPHE",*b"OUPHS",
	*b"OURIE",*b"OUSEL",*b"OUSTS",*b"OUTBY",*b"OUTDO",*b"OUTED",*b"OUTER",*b"OUTGO",*b"OUTRE",*b"OUTRO",
	*b"OUTTA",*b"OUZEL",*b"OUZOS",*b"OVALS",*b"OVARY",*b"OVATE",*b"OVELS",*b"OVENS",*b"OVERS",*b"OVERT",
	*b"OVINE",*b"OVIST",*b"OVOID",*b"OVOLI",*b"OVOLO",*b"OVULE",*b"OWCHE",*b"OWIES",*b"OWING",*b"OWLED",
	*b"OWLER",*b"OWLET",*b"OWNED",*b"OWNER",*b"OWRES",*b"OWRIE",*b"OWSEN",*b"OXBOW",*b"OXERS",*b"OXEYE",
	*b"OXIDE",*b"OXIDS",*b"OXIES",*b"OXIME",*b"OXIMS",*b"OXLIP",*b"OXTER",*b"OYERS",*b"OZEKI",*b"OZONE",
	*b"OZZIE",*b"PAALS",*b"PAANS",*b"PACAS",*b"PACED",*b"PACER",*b"PACES",*b"PACEY",*b"PACHA",*b"PACKS",
	*b"PACOS",*b"PACTA",*b"PACTS",*b"PADDY",*b"PADIS",*b"PADLE",*b"PADMA",*b"PADRE",*b"PADRI",*b"PAEAN",
	*b"PAEDO",*b"PAEON",*b"PAGAN",*b"PAGED",*b"PAGER",*b"PAGES",*b"PAGLE",*b"PAGOD",*b"PAGRI",*b"PAIKS",
	*b"PAILS",*b"PAINS",*b"PAINT",*b"PAIRE",*b"PAIRS",*b"PAISA",*b"PAISE",*b"PAKKA",*b"PALAS",*b"PALAY",
	*b"PALEA",*b"PALED",*b"PALER",*b"PALES",*b"PALET",*b"PALIS",*b"PALKI",*b"PALLA",*b"PALLS",*b"PALLY",
	*b"PALMS",*b"PALMY",*b"PALPI",*b"PALPS",*b"PALSA",*b"PALSY",*b"PAMPA",*b"PANAX",*b"PANCE",*b"PANDA",
	*b"PANDS",*b"PANDY",*b"PANED",*b"PANEL",*b"PANES",*b"PANGA",*b"PANGS",*b"PANIC",*b"PANIM",*b"PANKO",
	*b"PANNE",*b"PANNI",*b"PANSY",*b"PANTO",*b"PANTS",*b"PANTY",*b"PAOLI",*b"PAOLO",*b"PAPAL",*b"PAPAS",
	*b"PAPAW",*b"PAPER",*b"PAPES",*b"PAPPI",*b"PAPPY",*b"PARAE",*b"PARAS",*b"PARCH",*b"PARDI",*b"PARDS",
	*b"PARDY",*b"PARED",*b"PAREN",*b"PAREO",*b"PARER",*b"PARES",*b"PAREU",*b"PAREV",*b"PARGE",*b"PARGO",
	*b"PARIS",*b"PARKA",*b"PARKI",*b"PARKS",*b"PARKY",*b"PARLE",*b"PARLY",*b"PARMA",*b"PAROL",*b"PARPS",
	*b"PARRA",*b"PARRS",*b"PARRY",*b"PARSE",*b"PARTI",*b"PARTS",*b"PARTY",*b"PARVE",*b"PARVO",*b"PASEO",
	*b"PASES",*b"PASHA",*b"PASHM",*b"PASKA",*b"PASPY",*b"PASSE",*b"PASTA",*b"PASTE",*b"PASTS",*b"PASTY",
	*b"PATCH",*b"PATED",*b"PATEN",*b"PATER",*b"PATES",*b"PATHS",*b"PATIN",*b"PATIO",*b"PATKA",*b"PATLY",
	*b"PATSY",*b"PATTE",*b"PATTY",*b"PATUS",*b"PAUAS",*b"PAULS",*b"PAUSE",*b"PAVAN",*b"PAVED",*b"PAVEN",
	*b"PAVER",*b"PAVES",*b"PAVID",*b"PAVIN",*b"PAVIS",*b"PAWAS",*b"PAWAW",*b"PAWED",*b"PAWER",*b"PAWKS",
	*b"PAWKY",*b"PAWLS",*b"PAWNS",*b"PAXES",*b"PAYED",*b"PAYEE",*b"PAYER",*b"PAYOR",*b"PAYSD",*b"PEACE",
	*b"PEACH",*b"PEAGE",*b"PEAGS",*b"PEAKS",*b"PEAKY",*b"PEALS",*b"PEANS",*b"PEARE",*b"PEARL",*b"PEARS",
	*b"PEART",*b"PEASE",*b"PEATS",*b"PEATY",*b"PEAVY",*b"PEAZE",*b"PEBAS",*b"PECAN",*b"PECHS",*b"PECKE",
	*b"PECKS",*b"PECKY",*b"PEDAL",*b"PEDES",*b"PEDIS",*b"PEDRO",*b"PEECE",*b"PEEKS",*b"PEELS",*b"PEENS",
	*b"PEEOY",*b"PEEPE",*b"PEEPS",*b"PEERS",*b"PEERY",*b"PEEVE",*b"PEGGY",*b"PEGHS",*b"PEINS",*b"PEISE",
	*b"PEIZE",*b"PEKAN",*b"PEKES",*b"PEKIN",*b"PEKOE",*b"PELAS",*b"PELAU",*b"PELES",*b"PELFS",*b"PELLS",
	*b"PELMA",*b"PELON",*b"PELTA",*b"PELTS",*b"PENAL",*b"PENCE",*b"PENDS",*b"PENDU",*b"PENED",*b"PENES",
	*b"PENGO",*b"PENIE",*b"PENIS",*b"PENKS",*b"PENNA",*b"PENNE",*b"PENNI",*b"PENNY",*b"PENTS",*b"PEONS",
	*b"PEONY",*b"PEPLA",*b"PEPOS",*b"PEPPY",*b"PEPSI",*b"PERAI",*b"PERCE",*b"PERCH",*b"PERCS",*b"PERDU",
	*b"PERDY",*b"PEREA",*b"PERES",*b"PERIL",*b"PERIS",*b"PERKS",*b"PERKY",*b"PERMS",*b"PERNS",*b"PEROG",
	*b"PERPS",*b"PERRY",*b"PERSE",*b"PERST",*b"PERTS",*b"PERVE",*b"PERVO",*b"PERVS",*b"PERVY",*b"PESKY",
	*b"PESOS",*b"PESTO",*b"PESTS",*b"PESTY",*b"PETAL",*b"PETAR",*b"PETER",*b"PETIT",*b"PETRE",*b"PETRI",
	*b"PETTI",*b"PETTO",*b"PETTY",*b"PEWEE",*b"PEWIT",*b"PEYSE",*b"PHAGE",*b"PHANG",*b"PHARE",*b"PHARM",
	*b"PHASE",*b"PHEER",*b"PHENE",*b"PHEON",*b"PHESE",*b"PHIAL",*b"PHISH",*b"PHIZZ",*b"PHLOX",*b"PHOCA",
	*b"PHONE",*b"PHONO",*b"PHONS",*b"PHONY",*b"PHOTO",*b"PHOTS",*b"PHPHT",*b"PHUTS",*b"PHYLA",*b"PHYLE",
	*b"PIANI",*b"PIANO",*b"PIANS",*b"PIBAL",*b"PICAL",*b"PICAS",*b"PICCY",*b"PICKS",*b"PICKY",*b"PICOT",
	*b"PICRA",*b"PICUL",*b"PIECE",*b"PIEND",*b"PIERS",*b"PIERT",*b"PIETA",*b"PIETS",*b"PIETY",*b"PIEZO",
	*b"PIGGY",*b"PIGHT",*b"PIGMY",*b"PIING",*b"PIKAS",*b"PIKAU",*b"PIKED",*b"PIKER",*b"PIKES",*b"PIKEY",
	*b"PIKIS",*b"PIKUL",*b"PILAE",*b"PILAF",*b"PILAO",*b"PILAR",*b"PILAU",*b"PILAW",*b"PILCH",*b"PILEA",
	*b"PILED",*b"PILEI",*b"PILER",*b"PILES",*b"PILIS",*b"PILLS",*b"PILOT",*b"PILOW",*b"PILUM",*b"PILUS",
	*b"PIMAS",*b"PIMPS",*b"PINAS",*b"PINCH",*b"PINED",*b"PINES",*b"PINEY",*b"PINGO",*b"PINGS",*b"PINKO",
	*b"PINKS",*b"PINKY",*b"PINNA",*b"PINNY",*b"PINON",*b"PINOT",*b"PINTA",*b"PINTO",*b"PINTS",*b"PINUP",
	*b"PIONS",*b"PIONY",*b"PIOUS",*b"PIOYE",*b"PIOYS",*b"PIPAL",*b"PIPAS",*b"PIPED",*b"PIPER",*b"PIPES",
	*b"PIPET",*b"PIPIS",*b"PIPIT",*b"PIPPY",*b"PIPUL",*b"PIQUE",*b"PIRAI",*b"PIRLS",*b"PIRNS",*b"PIROG",
	*b"PISCO",*b"PISES",*b"PISKY",*b"PISOS",*b"PISSY",*b"PISTE",*b"PITAS",*b"PITCH",*b"PITHS",*b"PITHY",
	*b"PITON",*b"PITOT",*b"PITTA",*b"PIUMS",*b"PIVOT",*b"PIXEL",*b"PIXES",*b"PIXIE",*b"PIZED",*b"PIZES",
	*b"PIZZA",*b"PLAAS",*b"PLACE",*b"PLACK",*b"PLAGE",*b"PLAID",*b"PLAIN",*b"PLAIT",*b"PLANE",*b"PLANK",
	*b"PLANS",*b"PLANT",*b"PLAPS",*b"PLASH",*b"PLASM",*b"PLAST",*b"PLATE",*b"PLATS",*b"PLATT",*b"PLATY",
	*b"PLAYA",*b"PLAYS",*b"PLAZA",*b"PLEAD",*b"PLEAS",*b"PLEAT",*b"PLEBE",*b"PLEBS",*b"PLENA",*b"PLEON",
	*b"PLESH",*b"PLEWS",*b"PLICA",*b"PLIED",*b"PLIER",*b"PLIES",*b"PLIMS",*b"PLING",*b"PLINK",*b"PLOAT",
	*b"PLODS",*b"PLONG",*b"PLONK",*b"PLOOK",*b"PLOPS",*b"PLOTS",*b"PLOTZ",*b"PLOUK",*b"PLOWS",*b"PLOYE",
	*b"PLOYS",*b"PLUCK",*b"PLUES",*b"PLUFF",*b"PLUGS",*b"PLUMB",*b"PLUME",*b"PLUMP",*b"PLUMS",*b"PLUMY",
	*b"PLUNK",*b"PLUOT",*b"PLUSH",*b"PLUTO",*b"PLYER",*b"POACH",*b"POAKA",*b"POAKE",*b"POBOY",*b"POCKS",
	*b"POCKY",*b"PODAL",*b"PODDY",*b"PODEX",*b"PODGE",*b"PODGY",*b"PODIA",*b"POEMS",*b"POEPS",*b"POESY",
	*b"POETS",*b"POGEY",*b"POGGE",*b"POGOS",*b"POHED",*b"POILU",*b"POIND",*b"POINT",*b"POISE",*b"POKAL",
	*b"POKED",*b"POKER",*b"POKES",*b"POKEY",*b"POKIE",*b"POLAR",*b"POLED",*b"POLER",*b"POLES",*b"POLEY",
	*b"POLIO",*b"POLIS",*b"POLJE",*b"POLKA",*b"POLKS",*b"POLLS",*b"POLLY",*b"POLOS",*b"POLTS",*b"POLYP",
	*b"POLYS",*b"POMBE",*b"POMES",*b"POMMY",*b"POMOS",*b"POMPS",*b"PONCE",*b"PONCY",*b"PONDS",*b"PONES",
	*b"PONEY",*b"PONGA",*b"PONGO",*b"PONGS",*b"PONGY",*b"PONKS",*b"PONTS",*b"PONTY",*b"PONZU",*b"POOCH",
	*b"POODS",*b"POOED",*b"POOFS",*b"POOFY",*b"POOHS",*b"POOJA",*b"POOKA",*b"POOKS",*b"POOLS",*b"POONS",
	*b"POOPS",*b"POOPY",*b"POORI",*b"POORT",*b"POOTS",*b"POOVE",*b"POOVY",*b"POPES",*b"POPPA",*b"POPPY",
	*b"POPSY",*b"PORAE",*b"PORAL",*b"PORCH",*b"PORED",*b"PORER",*b"PORES",*b"PORGE",*b"PORGY",*b"PORIN",
	*b"PORKS",*b"PORKY",*b"PORNO",*b"PORNS",*b"PORNY",*b"PORTA",*b"PORTS",*b"PORTY",*b"POSED",*b"POSER",
	*b"POSES",*b"POSEY",*b"POSHO",*b"POSIT",*b"POSSE",*b"POSTS",*b"POTAE",*b"POTCH",*b"POTED",*b"POTES",
	*b"POTIN",*b"POTOO",*b"POTSY",*b"POTTO",*b"POTTS",*b"POTTY",*b"POUCH",*b"POUFF",*b"POUFS",*b"POUKE",
	*b"POUKS",*b"POULE",*b"POULP",*b"POULT",*b"POUND",*b"POUPE",*b"POUPT",*b"POURS",*b"POUTS",*b"POUTY",
	*b"POWAN",*b"POWER",*b"POWIN",*b"POWND",*b"POWNS",*b"POWNY",*b"POWRE",*b"POXED",*b"POXES",*b"POYNT",
	*b"POYOU",*b"POYSE",*b"POZZY",*b"PRAAM",*b"PRADS",*b"PRAHU",*b"PRAMS",*b"PRANA",*b"PRANG",*b"PRANK",
	*b"PRAOS",*b"PRASE",*b"PRATE",*b"PRATS",*b"PRATT",*b"PRATY",*b"PRAUS",*b"PRAWN",*b"PRAYS",*b"PREDY",
	*b"PREED",*b"PREEN",*b"PREES",*b"PREIF",*b"PREMS",*b"PREMY",*b"PRENT",*b"PREON",*b"PREOP",*b"PREPS",
	*b"PRESA",*b"PRESE",*b"PRESS",*b"PREST",*b"PREVE",*b"PREXY",*b"PREYS",*b"PRIAL",*b"PRICE",*b"PRICK",
	*b"PRICY",*b"PRIDE",*b"PRIED",*b"PRIEF",*b"PRIER",*b"PRIES",*b"PRIGS",*b"PRILL",*b"PRIMA",*b"PRIME",
	*b"PRIMI",*b"PRIMO",*b"PRIMP",*b"PRIMS",*b"PRIMY",*b"PRINK",*b"PRINT",*b"PRION",*b"PRIOR",*b"PRISE",
	*b"PRISM",*b"PRISS",*b"PRIVY",*b"PRIZE",*b"PROAS",*b"PROBE",*b"PROBS",*b"PRODS",*b"PROEM",*b"PROFS",
	*b"PROGS",*b"PROIN",*b"PROKE",*b"PROLE",*b"PROLL",*b"PROMO",*b"PROMS",*b"PRONE",*b"PRONG",*b"PRONK",
	*b"PROOF",*b"PROPS",*b"PRORE",*b"PROSE",*b"PROSO",*b"PROSS",*b"PROST",*b"PROSY",*b"PROTO",*b"PROUD",
	*b"PROUL",*b"PROVE",*b"PROWL",*b"PROWS",*b"PROXY",*b"PROYN",*b"PRUDE",*b"PRUNE",*b"PRUNT",*b"PRUTA",
	*b"PRYER",*b"PRYSE",*b"PSALM",*b"PSEUD",*b"PSHAW",*b"PSION",*b"PSOAE",*b"PSOAI",*b"PSOAS",*b"PSORA",
	*b"PSYCH",*b"PSYOP",*b"PUBCO",*b"PUBES",*b"PUBIC",*b"PUBIS",*b"PUCAN",*b"PUCER",*b"PUCES",*b"PUCKA",
	*b"PUCKS",*b"PUDDY",*b"PUDGE",*b"PUDGY",*b"PUDIC",*b"PUDOR",*b"PUDSY",*b"PUDUS",*b"PUERS",*b"PUFFA",
	*b"PUFFS",*b"PUFFY",*b"PUGGY",*b"PUGIL",*b"PUHAS",*b"PUJAH",*b"PUJAS",*b"PUKAS",*b"PUKED",*b"PUKER",
	*b"PUKES",*b"PUKEY",*b"PUKKA",*b"PUKUS",*b"PULAO",*b"PULAS",*b"PULED",*b"PULER",*b"PULES",*b"PULIK",
	*b"PULIS",*b"PULKA",*b"PULKS",*b"PULLI",*b"PULLS",*b"PULLY",*b"PULMO",*b"PULPS",*b"PULPY",*b"PULSE",
	*b"PULUS",*b"PUMAS",*b"PUMIE",*b"PUMPS",*b"PUNAS",*b"PUNCE",*b"PUNCH",*b"PUNGA",*b"PUNGS",*b"PUNJI",
	*b"PUNKA",*b"PUNKS",*b"PUNKY",*b"PUNNY",*b"PUNTO",*b"PUNTS",*b"PUNTY",*b"PUPAE",*b"PUPAS",*b"PUPIL",
	*b"PUPPY",*b"PUPUS",*b"PURDA",*b"PURED",*b"PUREE",*b"PURER",*b"PURES",*b"PURGE",*b"PURIN",*b"PURIS",
	*b"PURLS",*b"PURPY",*b"PURRS",*b"PURSE",*b"PURSY",*b"PURTY",*b"PUSES",*b"PUSHY",*b"PUSLE",*b"PUSSY",
	*b"PUTID",*b"PUTON",*b"PUTTI",*b"PUTTO",*b"PUTTS",*b"PUTTY",*b"PUZEL",*b"PWNED",*b"PYATS",*b"PYETS",
	*b"PYGAL",*b"PYGMY",*b"PYINS",*b"PYLON",*b"PYNED",*b"PYNES",*b"PYOID",*b"PYOTS",*b"PYRAL",*b"PYRAN",
	*b"PYRES",*b"PYREX",*b"PYRIC",*b"PYROS",*b"PYXED",*b"PYXES",*b"PYXIE",*b"PYXIS",*b"PZAZZ",*b"QADIS",
	*b"QAIDS",*b"QAJAQ",*b"QANAT",*b"QAPIK",*b"QIBLA",*b"QOPHS",*b"QORMA",*b"QUACK",*b"QUADS",*b"QUAFF",
	*b"QUAGS",*b"QUAIL",*b"QUAIR",*b"QUAIS",*b"QUAKE",*b"QUAKY",*b"QUALE",*b"QUALM",*b"QUANT",*b"QUARE",
	*b"QUARK",*b"QUART",*b"QUASH",*b"QUASI",*b"QUASS",*b"QUATE",*b"QUATS",*b"QUAYD",*b"QUAYS",*b"QUBIT",
	*b"QUEAN",*b"QUEEN",*b"QUEER",*b"QUELL",*b"QUEME",*b"QUENA",*b"QUERN",*b"QUERY",*b"QUEST",*b"QUEUE",
	*b"QUEYN",*b"QUEYS",*b"QUICH",*b"QUICK",*b"QUIDS",*b"QUIET",*b"QUIFF",*b"QUILL",*b"QUILT",*b"QUIMS",
	*b"QUINA",*b"QUINE",*b"QUINO",*b"QUINS",*b"QUINT",*b"QUIPO",*b"QUIPS",*b"QUIPU",*b"QUIRE",*b"QUIRK",
	*b"QUIRT",*b"QUIST",*b"QUITE",*b"QUITS",*b"QUOAD",*b"QUODS",*b"QUOIF",*b"QUOIN",*b"QUOIT",*b"QUOLL",
	*b"QUONK",*b"QUOPS",*b"QUOTA",*b"QUOTE",*b"QUOTH",*b"QURSH",*b"QUYTE",*b"RABAT",*b"RABBI",*b"RABIC",
	*b"RABID",*b"RABIS",*b"RACED",*b"RACER",*b"RACES",*b"RACHE",*b"RACKS",*b"RACON",*b"RADAR",*b"RADGE",
	*b"RADII",*b"RADIO",*b"RADIX",*b"RADON",*b"RAFFS",*b"RAFTS",*b"RAGAS",*b"RAGDE",*b"RAGED",*b"RAGEE",
	*b"RAGER",*b"RAGES",*b"RAGGA",*b"RAGGS",*b"RAGGY",*b"RAGIS",*b"RAGUS",*b"RAHED",*b"RAHUI",*b"RAIAS",
	*b"RAIDS",*b"RAIKS",*b"RAILE",*b"RAILS",*b"RAINE",*b"RAINS",*b"RAINY",*b"RAIRD",*b"RAISE",*b"RAITA",
	*b"RAITS",*b"RAJAH",*b"RAJAS",*b"RAJES",*b"RAKED",*b"RAKEE",*b"RAKER",*b"RAKES",*b"RAKIA",*b"RAKIS",
	*b"RAKUS",*b"RALES",*b"RALLY",*b"RALPH",*b"RAMAL",*b"RAMEE",*b"RAMEN",*b"RAMET",*b"RAMIE",*b"RAMIN",
	*b"RAMIS",*b"RAMMY",*b"RAMPS",*b"RAMUS",*b"RANAS",*b"RANCE",*b"RANCH",*b"RANDS",*b"RANDY",*b"RANEE",
	*b"RANGA",*b"RANGE",*b"RANGI",*b"RANGS",*b"RANGY",*b"RANID",*b"RANIS",*b"RANKE",*b"RANKS",*b"RANTS",
	*b"RAPED",*b"RAPER",*b"RAPES",*b"RAPHE",*b"RAPID",*b"RAPPE",*b"RARED",*b"RAREE",*b"RARER",*b"RARES",
	*b"RARKS",*b"RASED",*b"RASER",*b"RASES",*b"RASPS",*b"RASPY",*b"RASSE",*b"RASTA",*b"RATAL",*b"RATAN",
	*b"RATAS",*b"RATCH",*b"RATED",*b"RATEL",*b"RATER",*b"RATES",*b"RATHA",*b"RATHE",*b"RATHS",*b"RATIO",
	*b"RATOO",*b"RATOS",*b"RATTY",*b"RATUS",*b"RAUNS",*b"RAUPO",*b"RAVED",*b"RAVEL",*b"RAVEN",*b"RAVER",
	*b"RAVES",*b"RAVEY",*b"RAVIN",*b"RAWER",*b"RAWIN",*b"RAWLY",*b"RAWNS",*b"RAXED",*b"RAXES",*b"RAYAH",
	*b"RAYAS",*b"RAYED",*b"RAYLE",*b"RAYNE",*b"RAYON",*b"RAZED",*b"RAZEE",*b"RAZER",*b"RAZES",*b"RAZOO",
	*b"RAZOR",*b"REACH",*b"REACT",*b"READD",*b"READS",*b"READY",*b"REAIS",*b"REAKS",*b"REALM",*b"REALO",
	*b"REALS",*b"REAME",*b"REAMS",*b"REAMY",*b"REANS",*b"REAPS",*b"REARM",*b"REARS",*b"REAST",*b"REATA",
	*b"REATE",*b"REAVE",*b"REBAR",*b"REBBE",*b"REBEC",*b"REBEL",*b"REBID",*b"REBIT",*b"REBOP",*b"REBUS",
	*b"REBUT",*b"REBUY",*b"RECAL",*b"RECAP",*b"RECCE",*b"RECCO",*b"RECCY",*b"RECIT",*b"RECKS",*b"RECON",
	*b"RECTA",*b"RECTI",*b"RECTO",*b"RECUR",*b"RECUT",*b"REDAN",*b"REDDS",*b"REDDY",*b"REDED",*b"REDES",
	*b"REDIA",*b"REDID",*b"REDIP",*b"REDLY",*b"REDON",*b"REDOS",*b"REDOX",*b"REDRY",*b"REDUB",*b"REDUX",
	*b"REDYE",*b"REECH",*b"REEDE",*b"REEDS",*b"REEDY",*b"REEFS",*b"REEFY",*b"REEKS",*b"REEKY",*b"REELS",
	*b"REENS",*b"REEST",*b"REEVE",*b"REFED",*b"REFEL",*b"REFER",*b"REFFO",*b"REFIS",*b"REFIT",*b"REFIX",
	*b"REFLY",*b"REFRY",*b"REGAL",*b"REGAR",*b"REGES",*b"REGGO",*b"REGIE",*b"REGMA",*b"REGNA",*b"REGOS",
	*b"REGUR",*b"REHAB",*b"REHEM",*b"REIFS",*b"REIFY",*b"REIGN",*b"REIKI",*b"REIKS",*b"REINK",*b"REINS",
	*b"REIRD",*b"REIST",*b"REIVE",*b"REJIG",*b"REJON",*b"REKED",*b"REKES",*b"REKEY",*b"RELAX",*b"RELAY",
	*b"RELET",*b"RELIC",*b"RELIE",*b"RELIT",*b"RELLO",*b"REMAN",*b"REMAP",*b"REMEN",*b"REMET",*b"REMEX",
	*b"REMIT",*b"REMIX",*b"RENAL",*b"RENAY",*b"RENDS",*b"RENEW",*b"RENEY",*b"RENGA",*b"RENIG",*b"RENIN",
	*b"RENNE",*b"RENOS",*b"RENTE",*b"RENTS",*b"REOIL",*b"REORG",*b"REPAY",*b"REPEG",*b"REPEL",*b"REPIN",
	*b"REPLA",*b"REPLY",*b"REPOS",*b"REPOT",*b"REPPS",*b"REPRO",*b"RERAN",*b"RERIG",*b"RERUN",*b"RESAT",
	*b"RESAW",*b"RESAY",*b"RESEE",*b"RESES",*b"RESET",*b"RESEW",*b"RESID",*b"RESIN",*b"RESIT",*b"RESOD",
	*b"RESOW",*b"RESTO",*b"RESTS",*b"RESTY",*b"RESUS",*b"RETAG",*b"RETAX",*b"RETCH",*b"RETEM",*b"RETIA",
	*b"RETIE",*b"RETOX",*b"RETRO",*b"RETRY",*b"REUSE",*b"REVEL",*b"REVET",*b"REVIE",*b"REVUE",*b"REWAN",
	*b"REWAX",*b"REWED",*b"REWET",*b"REWIN",*b"REWON",*b"REWTH",*b"REXES",*b"REZES",*b"RHEAS",*b"RHEME",
	*b"RHEUM",*b"RHIES",*b"RHIME",*b"RHINE",*b"RHINO",*b"RHODY",*b"RHOMB",*b"RHONE",*b"RHUMB",*b"RHYME",
	*b"RHYNE",*b"RHYTA",*b"RIADS",*b"RIALS",*b"RIANT",*b"RIATA",*b"RIBAS",*b"RIBBY",*b"RIBES",*b"RICED",
	*b"RICER",*b"RICES",*b"RICEY",*b"RICHT",*b"RICIN",*b"RICKS",*b"RIDER",*b"RIDES",*b"RIDGE",*b"RIDGY",
	*b"RIDIC",*b"RIELS",*b"RIEMS",*b"RIEVE",*b"RIFER",*b"RIFFS",*b"RIFLE",*b"RIFTE",*b"RIFTS",*b"RIFTY",
	*b"RIGGS",*b"RIGHT",*b"RIGID",*b"RIGOL",*b"RIGOR",*b"RILED",*b"RILES",*b"RILEY",*b"RILLE",*b"RILLS",
	*b"RIMAE",*b"RIMED",*b"RIMER",*b"RIMES",*b"RIMUS",*b"RINDS",*b"RINDY",*b"RINES",*b"RINGS",*b"RINKS",
	*b"RINSE",*b"RIOJA",*b"RIOTS",*b"RIPED",*b"RIPEN",*b"RIPER",*b"RIPES",*b"RIPPS",*b"RISEN",*b"RISER",
	*b"RISES",*b"RISHI",*b"RISKS",*b"RISKY",*b"RISPS",*b"RISUS",*b"RITES",*b"RITTS",*b"RITZY",*b"RIVAL",
	*b"RIVAS",*b"RIVED",*b"RIVEL",*b"RIVEN",*b"RIVER",*b"RIVES",*b"RIVET",*b"RIYAL",*b"RIZAS",*b"ROACH",
	*b"ROADS",*b"ROAMS",*b"ROANS",*b"ROARS",*b"ROARY",*b"ROAST",*b"ROATE",*b"ROBED",*b"ROBES",*b"ROBIN",
	*b"ROBLE",*b"ROBOT",*b"ROCKS",*b"ROCKY",*b"RODED",*b"RODEO",*b"RODES",*b"ROGER",*b"ROGUE",*b"ROGUY",
	*b"ROHES",*b"ROIDS",*b"ROILS",*b"ROILY",*b"ROINS",*b"ROIST",*b"ROJAK",*b"ROJIS",*b"ROKED",*b"ROKER",
	*b"ROKES",*b"ROLAG",*b"ROLES",*b"ROLFS",*b"ROLLS",*b"ROMAL",*b"ROMAN",*b"ROMEO",*b"ROMPS",*b"RONDE",
	*b"RONDO",*b"RONEO",*b"RONES",*b"RONIN",*b"RONNE",*b"RONTE",*b"RONTS",*b"ROODS",*b"ROOFS",*b"ROOFY",
	*b"ROOKS",*b"ROOKY",*b"ROOMS",*b"ROOMY",*b"ROONS",*b"ROOPS",*b"ROOPY",*b"ROOSA",*b"ROOSE",*b"ROOST",
	*b"ROOTS",*b"ROOTY",*b"ROPED",*b"ROPER",*b"ROPES",*b"ROPEY",*b"ROQUE",*b"RORAL",*b"RORES",*b"RORIC",
	*b"RORID",*b"RORIE",*b"RORTS",*b"RORTY",*b"ROSED",*b"ROSES",*b"ROSET",*b"ROSHI",*b"ROSIN",*b"ROSIT",
	*b"ROSTI",*b"ROSTS",*b"ROTAL",*b"ROTAN",*b"ROTAS",*b"ROTCH",*b"ROTED",*b"ROTES",*b"ROTIS",*b"ROTLS",
	*b"ROTON",*b"ROTOR",*b"ROTOS",*b"ROTTE",*b"ROUEN",*b"ROUES",*b"ROUGE",*b"ROUGH",*b"ROULE",*b"ROULS",
	*b"ROUMS",*b"ROUND",*b"ROUPS",*b"ROUPY",*b"ROUSE",*b"ROUST",*b"ROUTE",*b"ROUTH",*b"ROUTS",*b"ROVED",
	*b"ROVEN",*b"ROVER",*b"ROVES",*b"ROWAN",*b"ROWDY",*b"ROWED",*b"ROWEL",*b"ROWEN",*b"ROWER",*b"ROWIE",
	*b"ROWME",*b"ROWND",*b"ROWTH",*b"ROWTS",*b"ROYAL",*b"ROYNE",*b"ROYST",*b"ROZET",*b"ROZIT",*b"RUANA",
	*b"RUBAI",*b"RUBBY",*b"RUBEL",*b"RUBES",*b"RUBIN",*b"RUBLE",*b"RUBLI",*b"RUBUS",*b"RUCHE",*b"RUCKS",
	*b"RUDAS",*b"RUDDS",*b"RUDDY",*b"RUDER",*b"RUDES",*b"RUDIE",*b"RUDIS",*b"RUEDA",*b"RUERS",*b"RUFFE",
	*b"RUFFS",*b"RUGAE",*b"RUGAL",*b"RUGBY",*b"RUGGY",*b"RUING",*b"RUINS",*b"RUKHS",*b"RULED",*b"RULER",
	*b"RULES",*b"RUMAL",*b"RUMBA",*b"RUMBO",*b"RUMEN",*b"RUMES",*b"RUMLY",*b"RUMMY",*b"RUMOR",*b"RUMPO",
	*b"RUMPS",*b"RUMPY",*b"RUNCH",*b"RUNDS",*b"RUNED",*b"RUNES",*b"RUNGS",*b"RUNIC",*b"RUNNY",*b"RUNTS",
	*b"RUNTY",*b"RUPEE",*b"RUPIA",*b"RURAL",*b"RURPS",*b"RURUS",*b"RUSAS",*b"RUSES",*b"RUSHY",*b"RUSKS",
	*b"RUSMA",*b"RUSSE",*b"RUSTS",*b"RUSTY",*b"RUTHS",*b"RUTIN",*b"RUTTY",*b"RYALS",*b"RYBAT",*b"RYKED",
	*b"RYKES",*b"RYMME",*b"RYNDS",*b"RYOTS",*b"RYPER",*b"SAAGS",*b"SABAL",*b"SABED",*b"SABER",*b"SABES",
	*b"SABHA",*b"SABIN",*b"SABIR",*b"SABLE",*b"SABOT",*b"SABRA",*b"SABRE",*b"SACKS",*b"SACRA",*b"SADDO",
	*b"SADES",*b"SADHE",*b"SADHU",*b"SADIS",*b"SADLY",*b"SADOS",*b"SADZA",*b"SAFED",*b"SAFER",*b"SAFES",
	*b"SAGAS",*b"SAGER",*b"SAGES",*b"SAGGY",*b"SAGOS",*b"SAGUM",*b"SAHEB",*b"SAHIB",*b"SAICE",*b"SAICK",
	*b"SAICS",*b"SAIDS",*b"SAIGA",*b"SAILS",*b"SAIMS",*b"SAINE",*b"SAINS",*b"SAINT",*b"SAIRS",*b"SAIST",
	*b"SAITH",*b"SAJOU",*b"SAKAI",*b"SAKER",*b"SAKES",*b"SAKIA",*b"SAKIS",*b"SAKTI",*b"SALAD",*b"SALAL",
	*b"SALAT",*b"SALEP",*b"SALES",*b"SALET",*b"SALIC",*b"SALIX",*b"SALLE",*b"SALLY",*b"SALMI",*b"SALOL",
	*b"SALON",*b"SALOP",*b"SALPA",*b"SALPS",*b"SALSA",*b"SALSE",*b"SALTO",*b"SALTS",*b"SALTY",*b"SALUE",
	*b"SALUT",*b"SALVE",*b"SALVO",*b"SAMAN",*b"SAMAS",*b"SAMBA",*b"SAMBO",*b"SAMEK",*b"SAMEL",*b"SAMEN",
	*b"SAMES",*b"SAMEY",*b"SAMFU",*b"SAMMY",*b"SAMPI",*b"SAMPS",*b"SANDS",*b"SANDY",*b"SANED",*b"SANER",
	*b"SANES",*b"SANGA",*b"SANGH",*b"SANGO",*b"SANGS",*b"SANKO",*b"SANSA",*b"SANTO",*b"SANTS",*b"SAOLA",
	*b"SAPAN",*b"SAPID",*b"SAPOR",*b"SAPPY",*b"SARAN",*b"SARDS",*b"SARED",*b"SAREE",*b"SARGE",*b"SARGO",
	*b"SARIN",*b"SARIS",*b"SARKS",*b"SARKY",*b"SAROD",*b"SAROS",*b"SARUS",*b"SASER",*b"SASIN",*b"SASSE",
	*b"SASSY",*b"SATAI",*b"SATAY",*b"SATED",*b"SATEM",*b"SATES",*b"SATIN",*b"SATIS",*b"SATYR",*b"SAUBA",
	*b"SAUCE",*b"SAUCH",*b"SAUCY",*b"SAUGH",*b"SAULS",*b"SAULT",*b"SAUNA",*b"SAUNT",*b"SAURY",*b"SAUTE",
	*b"SAUTS",*b"SAVED",*b"SAVER",*b"SAVES",*b"SAVEY",*b"SAVIN",*b"SAVOR",*b"SAVOY",*b"SAVVY",*b"SAWAH",
	*b"SAWED",*b"SAWER",*b"SAXES",*b"SAYED",*b"SAYER",*b"SAYID",*b"SAYNE",*b"SAYON",*b"SAYST",*b"SAZES",
	*b"SCABS",*b"SCADS",*b"SCAFF",*b"SCAGS",*b"SCAIL",*b"SCALA",*b"SCALD",*b"SCALE",*b"SCALL",*b"SCALP",
	*b"SCALY",*b"SCAMP",*b"SCAMS",*b"SCAND",*b"SCANS",*b"SCANT",*b"SCAPA",*b"SCAPE",*b"SCAPI",*b"SCARE",
	*b"SCARF",*b"SCARP",*b"SCARS",*b"SCART",*b"SCARY",*b"SCATH",*b"SCATS",*b"SCATT",*b"SCAUD",*b"SCAUP",
	*b"SCAUR",*b"SCAWS",*b"SCEAT",*b"SCENA",*b"SCEND",*b"SCENE",*b"SCENT",*b"SCHAV",*b"SCHMO",*b"SCHUL",
	*b"SCHWA",*b"SCION",*b"SCLIM",*b"SCODY",*b"SCOFF",*b"SCOGS",*b"SCOLD",*b"SCONE",*b"SCOOG",*b"SCOOP",
	*b"SCOOT",*b"SCOPA",*b"SCOPE",*b"SCOPS",*b"SCORE",*b"SCORN",*b"SCOTS",*b"SCOUG",*b"SCOUP",*b"SCOUR",
	*b"SCOUT",*b"SCOWL",*b"SCOWP",*b"SCOWS",*b"SCRAB",*b"SCRAE",*b"SCRAG",*b"SCRAM",*b"SCRAN",*b"SCRAP",
	*b"SCRAT",*b"SCRAW",*b"SCRAY",*b"SCREE",*b"SCREW",*b"SCRIM",*b"SCRIP",*b"SCROB",*b"SCROD",*b"SCROG",
	*b"SCROW",*b"SCRUB",*b"SCRUM",*b"SCUBA",*b"SCUDI",*b"SCUDO",*b"SCUDS",*b"SCUFF",*b"SCUFT",*b"SCUGS",
	*b"SCULK",*b"SCULL",*b"SCULP",*b"SCULS",*b"SCUMS",*b"SCUPS",*b"SCURF",*b"SCURS",*b"SCUSE",*b"SCUTA",
	*b"SCUTE",*b"SCUTS",*b"SCUZZ",*b"SCYES",*b"SDAYN",*b"SDEIN",*b"SEALS",*b"SEAME",*b"SEAMS",*b"SEAMY",
	*b"SEANS",*b"SEARE",*b"SEARS",*b"SEASE",*b"SEATS",*b"SEAZE",*b"SEBUM",*b"SECCO",*b"SECHS",*b"SECTS",
	*b"SEDAN",*b"SEDER",*b"SEDES",*b"SEDGE",*b"SEDGY",*b"SEDUM",*b"SEEDS",*b"SEEDY",*b"SEEKS",*b"SEELD",
	*b"SEELS",*b"SEELY",*b"SEEMS",*b"SEEPS",*b"SEEPY",*b"SEERS",*b"SEFER",*b"SEGAR",*b"SEGNI",*b"SEGNO",
	*b"SEGOL",*b"SEGOS",*b"SEGUE",*b"SEHRI",*b"SEIFS",*b"SEILS",*b"SEINE",*b"SEIRS",*b"SEISE",*b"SEISM",
	*b"SEITY",*b"SEIZA",*b"SEIZE",*b"SEKOS",*b"SEKTS",*b"SELAH",*b"SELES",*b"SELFS",*b"SELLA",*b"SELLE",
	*b"SELLS",*b"SELVA",*b"SEMEE",*b"SEMEN",*b"SEMES",*b"SEMIE",*b"SEMIS",*b"SENAS",*b"SENDS",*b"SENES",
	*b"SENGI",*b"SENNA",*b"SENOR",*b"SENSA",*b"SENSE",*b"SENSI",*b"SENTE",*b"SENTI",*b"SENTS",*b"SENVY",
	*b"SENZA",*b"SEPAD",*b"SEPAL",*b"SEPIA",*b"SEPIC",*b"SEPOY",*b"SEPTA",*b"SEPTS",*b"SERAC",*b"SERAI",
	*b"SERAL",*b"SERED",*b"SERER",*b"SERES",*b"SERFS",*b"SERGE",*b"SERIC",*b"SERIF",*b"SERIN",*b"SERKS",
	*b"SERON",*b"SEROW",*b"SERRA",*b"SERRE",*b"SERRS",*b"SERRY",*b"SERUM",*b"SERVE",*b"SERVO",*b"SESEY",
	*b"SESSA",*b"SETAE",*b"SETAL",*b"SETON",*b"SETTS",*b"SETUP",*b"SEVEN",*b"SEVER",*b"SEWAN",*b"SEWAR",
	*b"SEWED",*b"SEWEL",*b"SEWEN",*b"SEWER",*b"SEWIN",*b"SEXED",*b"SEXER",*b"SEXES",*b"SEXTO",*b"SEXTS",
	*b"SEYEN",*b"SHACK",*b"SHADE",*b"SHADS",*b"SHADY",*b"SHAFT",*b"SHAGS",*b"SHAHS",*b"SHAKE",*b"SHAKO",
	*b"SHAKT",*b"SHAKY",*b"SHALE",*b"SHALL",*b"SHALM",*b"SHALT",*b"SHALY",*b"SHAMA",*b"SHAME",*b"SHAMS",
	*b"SHAND",*b"SHANK",*b"SHANS",*b"SHAPE",*b"SHAPS",*b"SHARD",*b"SHARE",*b"SHARK",*b"SHARN",*b"SHARP",
	*b"SHASH",*b"SHAUL",*b"SHAVE",*b"SHAWL",*b"SHAWM",*b"SHAWN",*b"SHAWS",*b"SHAYA",*b"SHAYS",*b"SHCHI",
	*b"SHEAF",*b"SHEAL",*b"SHEAR",*b"SHEAS",*b"SHEDS",*b"SHEEL",*b"SHEEN",*b"SHEEP",*b"SHEER",*b"SHEET",
	*b"SHEIK",*b"SHELF",*b"SHELL",*b"SHEND",*b"SHENT",*b"SHEOL",*b"SHERD",*b"SHERE",*b"SHERO",*b"SHETS",
	*b"SHEVA",*b"SHEWN",*b"SHEWS",*b"SHIAI",*b"SHIED",*b"SHIEL",*b"SHIER",*b"SHIES",*b"SHIFT",*b"SHILL",
	*b"SHILY",*b"SHIMS",*b"SHINE",*b"SHINS",*b"SHINY",*b"SHIPS",*b"SHIRE",*b"SHIRK",*b"SHIRR",*b"SHIRS",
	*b"SHIRT",*b"SHISH",*b"SHISO",*b"SHIST",*b"SHITE",*b"SHITS",*b"SHIUR",*b"SHIVA",*b"SHIVE",*b"SHIVS",
	*b"SHLEP",*b"SHLUB",*b"SHMEK",*b"SHMOE",*b"SHOAL",*b"SHOAT",*b"SHOCK",*b"SHOED",*b"SHOER",*b"SHOES",
	*b"SHOGI",*b"SHOGS",*b"SHOJI",*b"SHOJO",*b"SHOLA",*b"SHONE",*b"SHOOK",*b"SHOOL",*b"SHOON",*b"SHOOS",
	*b"SHOOT",*b"SHOPE",*b"SHOPS",*b"SHORE",*b"SHORL",*b"SHORN",*b"SHORT",*b"SHOTE",*b"SHOTS",*b"SHOTT",
	*b"SHOUT",*b"SHOVE",*b"SHOWD",*b"SHOWN",*b"SHOWS",*b"SHOWY",*b"SHOYU",*b"SHRED",*b"SHREW",*b"SHRIS",
	*b"SHROW",*b"SHRUB",*b"SHRUG",*b"SHTIK",*b"SHTUM",*b"SHTUP",*b"SHUCK",*b"SHULE",*b"SHULN",*b"SHULS",
	*b"SHUNS",*b"SHUNT",*b"SHURA",*b"SHUSH",*b"SHUTE",*b"SHUTS",*b"SHWAS",*b"SHYER",*b"SHYLY",*b"SIALS",
	*b"SIBBS",*b"SIBYL",*b"SICES",*b"SICHT",*b"SICKO",*b"SICKS",*b"SICKY",*b"SIDAS",*b"SIDED",*b"SIDER",
	*b"SIDES",*b"SIDHA",*b"SIDHE",*b"SIDLE",*b"SIEGE",*b"SIELD",*b"SIENS",*b"SIENT",*b"SIETH",*b"SIEUR",
	*b"SIEVE",*b"SIFTS",*b"SIGHS",*b"SIGHT",*b"SIGIL",*b"SIGLA",*b"SIGMA",*b"SIGNA",*b"SIGNS",*b"SIJOS",
	*b"SIKAS",*b"SIKER",*b"SIKES",*b"SILDS",*b"SILED",*b"SILEN",*b"SILER",*b"SILES",*b"SILEX",*b"SILKS",
	*b"SILKY",*b"SILLS",*b"SILLY",*b"SILOS",*b"SILTS",*b"SILTY",*b"SILVA",*b"SIMAR",*b"SIMAS",*b"SIMBA",
	*b"SIMIS",*b"SIMPS",*b"SIMUL",*b"SINCE",*b"SINDS",*b"SINED",*b"SINES",*b"SINEW",*b"SINGE",*b"SINGS",
	*b"SINHS",*b"SINKS",*b"SINKY",*b"SINUS",*b"SIPED",*b"SIPES",*b"SIPPY",*b"SIRED",*b"SIREE",*b"SIREN",
	*b"SIRES",*b"SIRIH",*b"SIRIS",*b"SIROC",*b"SIRRA",*b"SIRUP",*b"SISAL",*b"SISES",*b"SISSY",*b"SISTA",
	*b"SISTS",*b"SITAR",*b"SITED",*b"SITES",*b"SITHE",*b"SITKA",*b"SITUP",*b"SITUS",*b"SIVER",*b"SIXER",
	*b"SIXES",*b"SIXMO",*b"SIXTE",*b"SIXTH",*b"SIXTY",*b"SIZAR",*b"SIZED",*b"SIZEL",*b"SIZER",*b"SIZES",
	*b"SKAGS",*b"SKAIL",*b"SKALD",*b"SKANK",*b"SKART",*b"SKATE",*b"SKATS",*b"SKATT",*b"SKAWS",*b"SKEAN",
	*b"SKEAR",*b"SKEDS",*b"SKEED",*b"SKEEF",*b"SKEEN",*b"SKEER",*b"SKEES",*b"SKEET",*b"SKEGG",*b"SKEGS",
	*b"SKEIN",*b"SKELF",*b"SKELL",*b"SKELM",*b"SKELP",*b"SKENE",*b"SKENS",*b"SKEOS",*b"SKEPS",*b"SKERS",
	*b"SKETS",*b"SKEWS",*b"SKIDS",*b"SKIED",*b"SKIER",*b"SKIES",*b"SKIEY",*b"SKIFF",*b"SKILL",*b"SKIMO",
	*b"SKIMP",*b"SKIMS",*b"SKINK",*b"SKINS",*b"SKINT",*b"SKIOS",*b"SKIPS",*b"SKIRL",*b"SKIRR",*b"SKIRT",
	*b"SKITE",*b"SKITS",*b"SKIVE",*b"SKIVY",*b"SKLIM",*b"SKOAL",*b"SKODY",*b"SKOFF",*b"SKOGS",*b"SKOLS",
	*b"SKOOL",*b"SKORT",*b"SKOSH",*b"SKRAN",*b"SKRIK",*b"SKUAS",*b"SKUGS",*b"SKULK",*b"SKULL",*b"SKUNK",
	*b"SKYED",*b"SKYER",*b"SKYEY",*b"SKYFS",*b"SKYRE",*b"SKYRS",*b"SKYTE",*b"SLABS",*b"SLACK",*b"SLADE",
	*b"SLAES",*b"SLAGS",*b"SLAID",*b"SLAIN",*b"SLAKE",*b"SLAMS",*b"SLANE",*b"SLANG",*b"SLANK",*b"SLANT",
	*b"SLAPS",*b"SLART",*b"SLASH",*b"SLATE",*b"SLATS",*b"SLATY",*b"SLAVE",*b"SLAWS",*b"SLAYS",*b"SLEBS",
	*b"SLEDS",*b"SLEEK",*b"SLEEP",*b"SLEER",*b"SLEET",*b"SLEPT",*b"SLEWS",*b"SLEYS",*b"SLICE",*b"SLICK",
	*b"SLIDE",*b"SLIER",*b"SLILY",*b"SLIME",*b"SLIMS",*b"SLIMY",*b"SLING",*b"SLINK",*b"SLIPE",*b"SLIPS",
	*b"SLIPT",*b"SLISH",*b"SLITS",*b"SLIVE",*b"SLOAN",*b"SLOBS",*b"SLOES",*b"SLOGS",*b"SLOID",*b"SLOJD",
	*b"SLOMO",*b"SLOOM",*b"SLOOP",*b"SLOOT",*b"SLOPE",*b"SLOPS",*b"SLOPY",*b"SLORM",*b"SLOSH",*b"SLOTH",
	*b"SLOTS",*b"SLOVE",*b"SLOWS",*b"SLOYD",*b"SLUBB",*b"SLUBS",*b"SLUED",*b"SLUES",*b"SLUFF",*b"SLUGS",
	*b"SLUIT",*b"SLUMP",*b"SLUMS",*b"SLUNG",*b"SLUNK",*b"SLURB",*b"SLURP",*b"SLURS",*b"SLUSE",*b"SLUSH",
	*b"SLUTS",*b"SLYER",*b"SLYLY",*b"SLYPE",*b"SMAAK",*b"SMACK",*b"SMAIK",*b"SMALL",*b"SMALM",*b"SMALT",
	*b"SMARM",*b"SMART",*b"SMASH",*b"SMAZE",*b"SMEAR",*b"SMEEK",*b"SMEES",*b"SMEIK",*b"SMEKE",*b"SMELL",
	*b"SMELT",*b"SMERK",*b"SMEWS",*b"SMILE",*b"SMIRK",*b"SMIRR",*b"SMIRS",*b"SMITE",*b"SMITH",*b"SMITS",
	*b"SMOCK",*b"SMOGS",*b"SMOKE",*b"SMOKO",*b"SMOKY",*b"SMOLT",*b"SMOOR",*b"SMOOT",*b"SMORE",*b"SMORG",
	*b"SMOTE",*b"SMOUT",*b"SMOWT",*b"SMUGS",*b"SMURS",*b"SMUSH",*b"SMUTS",*b"SNABS",*b"SNACK",*b"SNAFU",
	*b"SNAGS",*b"SNAIL",*b"SNAKE",*b"SNAKY",*b"SNAPS",*b"SNARE",*b"SNARF",*b"SNARK",*b"SNARL",*b"SNARS",
	*b"SNARY",*b"SNASH",*b"SNATH",*b"SNAWS",*b"SNEAD",*b"SNEAK",*b"SNEAP",*b"SNEBS",*b"SNECK",*b"SNEDS",
	*b"SNEED",*b"SNEER",*b"SNEES",*b"SNELL",*b"SNIBS",*b"SNICK",*b"SNIDE",*b"SNIES",*b"SNIFF",*b"SNIFT",
	*b"SNIGS",*b"SNIPE",*b"SNIPS",*b"SNIPY",*b"SNIRT",*b"SNITS",*b"SNOBS",*b"SNODS",*b"SNOEK",*b"SNOEP",
	*b"SNOGS",*b"SNOKE",*b"SNOOD",*b"SNOOK",*b"SNOOL",*b"SNOOP",*b"SNOOT",*b"SNORE",*b"SNORT",*b"SNOTS",
	*b"SNOUT",*b"SNOWK",*b"SNOWS",*b"SNOWY",*b"SNUBS",*b"SNUCK",*b"SNUFF",*b"SNUGS",*b"SNUSH",*b"SNYES",
	*b"SOAKS",*b"SOAPS",*b"SOAPY",*b"SOARE",*b"SOARS",*b"SOAVE",*b"SOBAS",*b"SOBER",*b"SOCAS",*b"SOCES",
	*b"SOCKO",*b"SOCKS",*b"SOCLE",*b"SODAS",*b"SODDY",*b"SODIC",*b"SODOM",*b"SOFAR",*b"SOFAS",*b"SOFTA",
	*b"SOFTS",*b"SOFTY",*b"SOGER",*b"SOGGY",*b"SOHUR",*b"SOILS",*b"SOILY",*b"SOJAS",*b"SOJUS",*b"SOKAH",
	*b"SOKEN",*b"SOKES",*b"SOKOL",*b"SOLAH",*b"SOLAN",*b"SOLAR",*b"SOLAS",*b"SOLDE",*b"SOLDI",*b"SOLDO",
	*b"SOLDS",*b"SOLED",*b"SOLEI",*b"SOLER",*b"SOLES",*b"SOLID",*b"SOLON",*b"SOLOS",*b"SOLUM",*b"SOLUS",
	*b"SOLVE",*b"SOMAN",*b"SOMAS",*b"SONAR",*b"SONCE",*b"SONDE",*b"SONES",*b"SONGS",*b"SONIC",*b"SONLY",
	*b"SONNE",*b"SONNY",*b"SONSE",*b"SONSY",*b"SOOEY",*b"SOOKS",*b"SOOKY",*b"SOOLE",*b"SOOLS",*b"SOOMS",
	*b"SOOPS",*b"SOOTE",*b"SOOTH",*b"SOOTS",*b"SOOTY",*b"SOPHS",*b"SOPHY",*b"SOPOR",*b"SOPPY",*b"SOPRA",
	*b"SORAL",*b"SORAS",*b"SORBO",*b"SORBS",*b"SORDA",*b"SORDO",*b"SORDS",*b"SORED",*b"SOREE",*b"SOREL",
	*b"SORER",*b"SORES",*b"SOREX",*b"SORGO",*b"SORNS",*b"SORRA",*b"SORRY",*b"SORTA",*b"SORTS",*b"SORUS",
	*b"SOTHS",*b"SOTOL",*b"SOUCE",*b"SOUCT",*b"SOUGH",*b"SOUKS",*b"SOULS",*b"SOUMS",*b"SOUND",*b"SOUPS",
	*b"SOUPY",*b"SOURS",*b"SOUSE",*b"SOUTH",*b"SOUTS",*b"SOWAR",*b"SOWCE",*b"SOWED",*b"SOWER",*b"SOWFF",
	*b"SOWFS",*b"SOWLE",*b"SOWLS",*b"SOWMS",*b"SOWND",*b"SOWNE",*b"SOWPS",*b"SOWSE",*b"SOWTH",*b"SOYAS",
	*b"SOYLE",*b"SOYUZ",*b"SOZIN",*b"SPACE",*b"SPACY",*b"SPADE",*b"SPADO",*b"SPAED",*b"SPAER",*b"SPAES",
	*b"SPAGS",*b"SPAHI",*b"SPAIL",*b"SPAIN",*b"SPAIT",*b"SPAKE",*b"SPALD",*b"SPALE",*b"SPALL",*b"SPALT",
	*b"SPAMS",*b"SPANE",*b"SPANG",*b"SPANK",*b"SPANS",*b"SPARD",*b"SPARE",*b"SPARK",*b"SPARS",*b"SPART",
	*b"SPASM",*b"SPATE",*b"SPATS",*b"SPAUL",*b"SPAWL",*b"SPAWN",*b"SPAWS",*b"SPAYD",*b"SPAYS",*b"SPAZA",
	*b"SPAZZ",*b"SPEAK",*b"SPEAL",*b"SPEAN",*b"SPEAR",*b"SPEAT",*b"SPECK",*b"SPECS",*b"SPECT",*b"SPEED",
	*b"SPEEL",*b"SPEER",*b"SPEIL",*b"SPEIR",*b"SPEKS",*b"SPELD",*b"SPELK",*b"SPELL",*b"SPELT",*b"SPEND",
	*b"SPENT",*b"SPEOS",*b"SPERM",*b"SPETS",*b"SPEUG",*b"SPEWS",*b"SPEWY",*b"SPIAL",*b"SPICA",*b"SPICE",
	*b"SPICK",*b"SPICS",*b"SPICY",*b"SPIDE",*b"SPIED",*b"SPIEL",*b"SPIER",*b"SPIES",*b"SPIFF",*b"SPIFS",
	*b"SPIKE",*b"SPIKS",*b"SPIKY",*b"SPILE",*b"SPILL",*b"SPILT",*b"SPIMS",*b"SPINA",*b"SPINE",*b"SPINK",
	*b"SPINS",*b"SPINY",*b"SPIRE",*b"SPIRT",*b"SPIRY",*b"SPITE",*b"SPITS",*b"SPITZ",*b"SPIVS",*b"SPLAT",
	*b"SPLAY",*b"SPLIT",*b"SPLOG",*b"SPODE",*b"SPODS",*b"SPOIL",*b"SPOKE",*b"SPOOF",*b"SPOOK",*b"SPOOL",
	*b"SPOOM",*b"SPOON",*b"SPOOR",*b"SPOOT",*b"SPORE",*b"SPORK",*b"SPORT",*b"SPOSH",*b"SPOTS",*b"SPOUT",
	*b"SPRAD",*b"SPRAG",*b"SPRAT",*b"SPRAY",*b"SPRED",*b"SPREE",*b"SPREW",*b"SPRIG",*b"SPRIT",*b"SPROD",
	*b"SPROG",*b"SPRUE",*b"SPRUG",*b"SPUDS",*b"SPUED",*b"SPUER",*b"SPUES",*b"SPUGS",*b"SPULE",*b"SPUME",
	*b"SPUMY",*b"SPUNK",*b"SPURN",*b"SPURS",*b"SPURT",*b"SPUTA",*b"SPYAL",*b"SPYRE",*b"SQUAB",*b"SQUAD",
	*b"SQUAT",*b"SQUAW",*b"SQUEG",*b"SQUIB",*b"SQUID",*b"SQUIT",*b"SQUIZ",*b"STABS",*b"STACK",*b"STADE",
	*b"STAFF",*b"STAGE",*b"STAGS",*b"STAGY",*b"STAID",*b"STAIG",*b"STAIN",*b"STAIR",*b"STAKE",*b"STALE",
	*b"STALK",*b"STALL",*b"STAMP",*b"STAND",*b"STANE",*b"STANG",*b"STANK",*b"STAPH",*b"STAPS",*b"STARE",
	*b"STARK",*b"STARN",*b"STARR",*b"STARS",*b"START",*b"STASH",*b"STATE",*b"STATS",*b"STAUN",*b"STAVE",
	*b"STAWS",*b"STAYS",*b"STEAD",*b"STEAK",*b"STEAL",*b"STEAM",*b"STEAN",*b"STEAR",*b"STEDD",*b"STEDE",
	*b"STEDS",*b"STEED",*b"STEEK",*b"STEEL",*b"STEEM",*b"STEEN",*b"STEEP",*b"STEER",*b"STEIL",*b"STEIN",
	*b"STELA",*b"STELE",*b"STELL",*b"STEME",*b"STEMS",*b"STEND",*b"STENO",*b"STENS",*b"STENT",*b"STEPS",
	*b"STEPT",*b"STERE",*b"STERN",*b"STETS",*b"STEWS",*b"STEWY",*b"STEYS",*b"STICH",*b"STICK",*b"STIED",
	*b"STIES",*b"STIFF",*b"STILB",*b"STILE",*b"STILL",*b"STILT",*b"STIME",*b"STIMS",*b"STIMY",*b"STING",
	*b"STINK",*b"STINT",*b"STIPA",*b"STIPE",*b"STIRE",*b"STIRK",*b"STIRP",*b"STIRS",*b"STIVE",*b"STIVY",
	*b"STOAE",*b"STOAI",*b"STOAS",*b"STOAT",*b"STOBS",*b"STOCK",*b"STOEP",*b"STOGY",*b"STOIC",*b"STOIT",
	*b"STOKE",*b"STOLE",*b"STOLN",*b"STOMA",*b"STOMP",*b"STOND",*b"STONE",*b"STONG",*b"STONK",*b"STONN",
	*b"STONY",*b"STOOD",*b"STOOK",*b"STOOL",*b"STOOP",*b"STOOR",*b"STOPE",*b"STOPS",*b"STOPT",*b"STORE",
	*b"STORK",*b"STORM",*b"STORY",*b"STOSS",*b"STOTS",*b"STOTT",*b"STOUN",*b"STOUP",*b"STOUR",*b"STOUT",
	*b"STOVE",*b"STOWN",*b"STOWP",*b"STOWS",*b"STRAD",*b"STRAE",*b"STRAG",*b"STRAK",*b"STRAP",*b"STRAW",
	*b"STRAY",*b"STREP",*b"STREW",*b"STRIA",*b"STRIG",*b"STRIM",*b"STRIP",*b"STROP",*b"STROW",*b"STROY",
	*b"STRUM",*b"STRUT",*b"STUBS",*b"STUCK",*b"STUDE",*b"STUDS",*b"STUDY",*b"STUFF",*b"STULL",*b"STULM",
	*b"STUMM",*b"STUMP",*b"STUMS",*b"STUNG",*b"STUNK",*b"STUNS",*b"STUNT",*b"STUPA",*b"STUPE",*b"STURE",
	*b"STURT",*b"STYED",*b"STYES",*b"STYLE",*b"STYLI",*b"STYLO",*b"STYME",*b"STYMY",*b"STYRE",*b"STYTE",
	*b"SUAVE",*b"SUBAH",*b"SUBAS",*b"SUBBY",*b"SUBER",*b"SUBHA",*b"SUCCI",*b"SUCKS",*b"SUCKY",*b"SUCRE",
	*b"SUDDS",*b"SUDOR",*b"SUDSY",*b"SUEDE",*b"SUENT",*b"SUERS",*b"SUETE",*b"SUETS",*b"SUETY",*b"SUGAN",
	*b"SUGAR",*b"SUGHS",*b"SUGOS",*b"SUHUR",*b"SUIDS",*b"SUING",*b"SUINT",*b"SUITE",*b"SUITS",*b"SUJEE",
	*b"SUKHS",*b"SUKUK",*b"SULCI",*b"SULFA",*b"SULFO",*b"SULKS",*b"SULKY",*b"SULLY",*b"SULPH",*b"SULUS",
	*b"SUMAC",*b"SUMIS",*b"SUMMA",*b"SUMOS",*b"SUMPH",*b"SUMPS",*b"SUNIS",*b"SUNKS",*b"SUNNA",*b"SUNNS",
	*b"SUNNY",*b"SUNUP",*b"SUPER",*b"SUPES",*b"SUPRA",*b"SURAH",*b"SURAL",*b"SURAS",*b"SURAT",*b"SURDS",
	*b"SURED",*b"SURER",*b"SURES",*b"SURFS",*b"SURFY",*b"SURGE",*b"SURGY",*b"SURLY",*b"SURRA",*b"SUSED",
	*b"SUSES",*b"SUSHI",*b"SUSUS",*b"SUTOR",*b"SUTRA",*b"SUTTA",*b"SWABS",*b"SWACK",*b"SWADS",*b"SWAGE",
	*b"SWAGS",*b"SWAIL",*b"SWAIN",*b"SWALE",*b"SWALY",*b"SWAMI",*b"SWAMP",*b"SWAMY",*b"SWANG",*b"SWANK",
	*b"SWANS",*b"SWAPS",*b"SWAPT",*b"SWARD",*b"SWARE",*b"SWARF",*b"SWARM",*b"SWART",*b"SWASH",*b"SWATH",
	*b"SWATS",*b"SWAYL",*b"SWAYS",*b"SWEAL",*b"SWEAR",*b"SWEAT",*b"SWEDE",*b"SWEED",*b"SWEEL",*b"SWEEP",
	*b"SWEER",*b"SWEES",*b"SWEET",*b"SWEIR",*b"SWELL",*b"SWELT",*b"SWEPT",*b"SWERF",*b"SWEYS",*b"SWIES",
	*b"SWIFT",*b"SWIGS",*b"SWILE",*b"SWILL",*b"SWIMS",*b"SWINE",*b"SWING",*b"SWINK",*b"SWIPE",*b"SWIRE",
	*b"SWIRL",*b"SWISH",*b"SWISS",*b"SWITH",*b"SWITS",*b"SWIVE",*b"SWIZZ",*b"SWOBS",*b"SWOLE",*b"SWOLN",
	*b"SWOON",*b"SWOOP",*b"SWOPS",*b"SWOPT",*b"SWORD",*b"SWORE",*b"SWORN",*b"SWOTS",*b"SWOUN",*b"SWUNG",
	*b"SYBBE",*b"SYBIL",*b"SYBOE",*b"SYBOW",*b"SYCEE",*b"SYCES",*b"SYCON",*b"SYENS",*b"SYKER",*b"SYKES",
	*b"SYLIS",*b"SYLPH",*b"SYLVA",*b"SYMAR",*b"SYNCH",*b"SYNCS",*b"SYNDS",*b"SYNED",*b"SYNES",*b"SYNOD",
	*b"SYNTH",*b"SYPED",*b"SYPES",*b"SYPHS",*b"SYRAH",*b"SYREN",*b"SYRUP",*b"SYSOP",*b"SYTHE",*b"SYVER",
	*b"TAALS",*b"TAATA",*b"TABBY",*b"TABER",*b"TABES",*b"TABID",*b"TABIS",*b"TABLA",*b"TABLE",*b"TABOO",
	*b"TABOR",*b"TABUN",*b"TABUS",*b"TACAN",*b"TACES",*b"TACET",*b"TACHE",*b"TACHO",*b"TACHS",*b"TACIT",
	*b"TACKS",*b"TACKY",*b"TACOS",*b"TACTS",*b"TAELS",*b"TAFFY",*b"TAFIA",*b"TAGGY",*b"TAGMA",*b"TAHAS",
	*b"TAHRS",*b"TAIGA",*b"TAIGS",*b"TAIKO",*b"TAILS",*b"TAINS",*b"TAINT",*b"TAIRA",*b"TAISH",*b"TAITS",
	*b"TAJES",*b"TAKAS",*b"TAKEN",*b"TAKER",*b"TAKES",*b"TAKHI",*b"TAKIN",*b"TAKIS",*b"TAKKY",*b"TALAK",
	*b"TALAQ",*b"TALAR",*b"TALAS",*b"TALCS",*b"TALCY",*b"TALEA",*b"TALER",*b"TALES",*b"TALKS",*b"TALKY",
	*b"TALLS",*b"TALLY",*b"TALMA",*b"TALON",*b"TALPA",*b"TALUK",*b"TALUS",*b"TAMAL",*b"TAMED",*b"TAMER",
	*b"TAMES",*b"TAMIN",*b"TAMIS",*b"TAMMY",*b"TAMPS",*b"TANAS",*b"TANGA",*b"TANGI",*b"TANGO",*b"TANGS",
	*b"TANGY",*b"TANHS",*b"TANKA",*b"TANKS",*b"TANKY",*b"TANNA",*b"TANSY",*b"TANTI",*b"TANTO",*b"TANTY",
	*b"TAPAS",*b"TAPED",*b"TAPEN",*b"TAPER",*b"TAPES",*b"TAPET",*b"TAPIR",*b"TAPIS",*b"TAPPA",*b"TAPUS",
	*b"TARAS",*b"TARDO",*b"TARDY",*b"TARED",*b"TARES",*b"TARGA",*b"TARGE",*b"TARNS",*b"TAROC",*b"TAROK",
	*b"TAROS",*b"TAROT",*b"TARPS",*b"TARRE",*b"TARRY",*b"TARSI",*b"TARTS",*b"TARTY",*b"TASAR",*b"TASED",
	*b"TASER",*b"TASES",*b"TASKS",*b"TASSA",*b"TASSE",*b"TASSO",*b"TASTE",*b"TASTY",*b"TATAR",*b"TATER",
	*b"TATES",*b"TATHS",*b"TATIE",*b"TATOU",*b"TATTS",*b"TATTY",*b"TATUS",*b"TAUBE",*b"TAULD",*b"TAUNT",
	*b"TAUON",*b"TAUPE",*b"TAUTS",*b"TAVAH",*b"TAVAS",*b"TAVER",*b"TAWAI",*b"TAWAS",*b"TAWED",*b"TAWER",
	*b"TAWIE",*b"TAWNY",*b"TAWSE",*b"TAWTS",*b"TAXED",*b"TAXER",*b"TAXES",*b"TAXIS",*b"TAXOL",*b"TAXON",
	*b"TAXOR",*b"TAXUS",*b"TAYRA",*b"TAZZA",*b"TAZZE",*b"TEACH",*b"TEADE",*b"TEADS",*b"TEAED",*b"TEAKS",
	*b"TEALS",*b"TEAMS",*b"TEARS",*b"TEARY",*b"TEASE",*b"TEATS",*b"TEAZE",*b"TECHS",*b"TECHY",*b"TECTA",
	*b"TEDDY",*b"TEELS",*b"TEEMS",*b"TEEND",*b"TEENE",*b"TEENS",*b"TEENY",*b"TEERS",*b"TEETH",*b"TEFFS",
	*b"TEGGS",*b"TEGUA",*b"TEGUS",*b"TEHRS",*b"TEIID",*b"TEILS",*b"TEIND",*b"TEINS",*b"TELAE",*b"TELCO",
	*b"TELES",*b"TELEX",*b"TELIA",*b"TELIC",*b"TELLS",*b"TELLY",*b"TELOI",*b"TELOS",*b"TEMED",*b"TEMES",
	*b"TEMPI",*b"TEMPO",*b"TEMPS",*b"TEMPT",*b"TEMSE",*b"TENCH",*b"TENDS",*b"TENDU",*b"TENES",*b"TENET",
	*b"TENGE",*b"TENIA",*b"TENNE",*b"TENNO",*b"TENNY",*b"TENON",*b"TENOR",*b"TENSE",*b"TENTH",*b"TENTS",
	*b"TENTY",*b"TENUE",*b"TEPAL",*b"TEPAS",*b"TEPEE",*b"TEPID",*b"TEPOY",*b"TERAI",*b"TERAS",*b"TERCE",
	*b"TEREK",*b"TERES",*b"TERFE",*b"TERFS",*b"TERGA",*b"TERMS",*b"TERNE",*b"TERNS",*b"TERRA",*b"TERRY",
	*b"TERSE",*b"TERTS",*b"TESLA",*b"TESTA",*b"TESTE",*b"TESTS",*b"TESTY",*b"TETES",*b"TETHS",*b"TETRA",
	*b"TETRI",*b"TEUCH",*b"TEUGH",*b"TEWED",*b"TEWEL",*b"TEWIT",*b"TEXAS",*b"TEXES",*b"TEXTS",*b"THACK",
	*b"THAGI",*b"THAIM",*b"THALE",*b"THALI",*b"THANA",*b"THANE",*b"THANG",*b"THANK",*b"THANS",*b"THANX",
	*b"THARM",*b"THARS",*b"THAWS",*b"THAWY",*b"THEBE",*b"THECA",*b"THEED",*b"THEEK",*b"THEES",*b"THEFT",
	*b"THEGN",*b"THEIC",*b"THEIN",*b"THEIR",*b"THELF",*b"THEMA",*b"THEME",*b"THENS",*b"THEOW",*b"THERE",
	*b"THERM",*b"THESE",*b"THESP",*b"THETA",*b"THETE",*b"THEWS",*b"THEWY",*b"THICK",*b"THIEF",*b"THIGH",
	*b"THIGS",*b"THILK",*b"THILL",*b"THINE",*b"THING",*b"THINK",*b"THINS",*b"THIOL",*b"THIRD",*b"THIRL",
	*b"THOFT",*b"THOLE",*b"THOLI",*b"THONG",*b"THORN",*b"THORO",*b"THORP",*b"THOSE",*b"THOUS",*b"THOWL",
	*b"THRAE",*b"THRAW",*b"THREE",*b"THREW",*b"THRID",*b"THRIP",*b"THROB",*b"THROE",*b"THROW",*b"THRUM",
	*b"THUDS",*b"THUGS",*b"THUJA",*b"THUMB",*b"THUMP",*b"THUNK",*b"THURL",*b"THUYA",*b"THYME",*b"THYMI",
	*b"THYMY",*b"TIANS",*b"TIARA",*b"TIARS",*b"TIBIA",*b"TICAL",*b"TICCA",*b"TICED",*b"TICES",*b"TICHY",
	*b"TICKS",*b"TICKY",*b"TIDAL",*b"TIDDY",*b"TIDED",*b"TIDES",*b"TIERS",*b"TIFFS",*b"TIFOS",*b"TIFTS",
	*b"TIGER",*b"TIGES",*b"TIGHT",*b"TIGON",*b"TIKAS",*b"TIKES",*b"TIKIS",*b"TIKKA",*b"TILAK",*b"TILDE",
	*b"TILED",*b"TILER",*b"TILES",*b"TILLS",*b"TILLY",*b"TILTH",*b"TILTS",*b"TIMBO",*b"TIMED",*b"TIMER",
	*b"TIMES",*b"TIMID",*b"TIMON",*b"TIMPS",*b"TINAS",*b"TINCT",*b"TINDS",*b"TINEA",*b"TINED",*b"TINES",
	*b"TINGE",*b"TINGS",*b"TINKS",*b"TINNY",*b"TINTS",*b"TINTY",*b"TIPIS",*b"TIPPY",*b"TIPSY",*b"TIRED",
	*b"TIRES",*b"TIRLS",*b"TIROS",*b"TIRRS",*b"TITAN",*b"TITCH",*b"TITER",*b"TITHE",*b"TITIS",*b"TITLE",
	*b"TITRE",*b"TITTY",*b"TITUP",*b"TIYIN",*b"TIYNS",*b"TIZES",*b"TIZZY",*b"TOADS",*b"TOADY",*b"TOAST",
	*b"TOAZE",*b"TOCKS",*b"TOCKY",*b"TOCOS",*b"TODAY",*b"TODDE",*b"TODDY",*b"TOEAS",*b"TOFFS",*b"TOFFY",
	*b"TOFTS",*b"TOFUS",*b"TOGAE",*b"TOGAS",*b"TOGED",*b"TOGES",*b"TOGUE",*b"TOHOS",*b"TOILE",*b"TOILS",
	*b"TOING",*b"TOISE",*b"TOITS",*b"TOKAY",*b"TOKED",*b"TOKEN",*b"TOKER",*b"TOKES",*b"TOKOS",*b"TOLAN",
	*b"TOLAR",*b"TOLAS",*b"TOLED",*b"TOLES",*b"TOLLS",*b"TOLLY",*b"TOLTS",*b"TOLUS",*b"TOLYL",*b"TOMAN",
	*b"TOMBS",*b"TOMES",*b"TOMIA",*b"TOMMY",*b"TOMOS",*b"TONAL",*b"TONDI",*b"TONDO",*b"TONED",*b"TONER",
	*b"TONES",*b"TONEY",*b"TONGA",*b"TONGS",*b"TONIC",*b"TONKA",*b"TONKS",*b"TONNE",*b"TONUS",*b"TOOLS",
	*b"TOOMS",*b"TOONS",*b"TOOTH",*b"TOOTS",*b"TOPAZ",*b"TOPED",*b"TOPEE",*b"TOPEK",*b"TOPER",*b"TOPES",
	*b"TOPHE",*b"TOPHI",*b"TOPHS",*b"TOPIC",*b"TOPIS",*b"TOPOI",*b"TOPOS",*b"TOPPY",*b"TOQUE",*b"TORAH",
	*b"TORAN",*b"TORAS",*b"TORCH",*b"TORCS",*b"TORES",*b"TORIC",*b"TORII",*b"TOROS",*b"TOROT",*b"TORRS",
	*b"TORSE",*b"TORSI",*b"TORSK",*b"TORSO",*b"TORTA",*b"TORTE",*b"TORTS",*b"TORUS",*b"TOSAS",*b"TOSED",
	*b"TOSES",*b"TOSHY",*b"TOSSY",*b"TOTAL",*b"TOTED",*b"TOTEM",*b"TOTER",*b"TOTES",*b"TOTTY",*b"TOUCH",
	*b"TOUGH",*b"TOUKS",*b"TOUNS",*b"TOURS",*b"TOUSE",*b"TOUSY",*b"TOUTS",*b"TOUZE",*b"TOUZY",*b"TOWED",
	*b"TOWEL",*b"TOWER",*b"TOWIE",*b"TOWNS",*b"TOWNY",*b"TOWSE",*b"TOWSY",*b"TOWTS",*b"TOWZE",*b"TOWZY",
	*b"TOXIC",*b"TOXIN",*b"TOYED",*b"TOYER",*b"TOYON",*b"TOYOS",*b"TOZED",*b"TOZES",*b"TOZIE",*b"TRABS",
	*b"TRACE",*b"TRACK",*b"TRACT",*b"TRADE",*b"TRADS",*b"TRAGI",*b"TRAIK",*b"TRAIL",*b"TRAIN",*b"TRAIT",
	*b"TRAMP",*b"TRAMS",*b"TRANK",*b"TRANQ",*b"TRANS",*b"TRANT",*b"TRAPE",*b"TRAPS",*b"TRAPT",*b"TRASH",
	*b"TRASS",*b"TRATS",*b"TRATT",*b"TRAVE",*b"TRAWL",*b"TRAYF",*b"TRAYS",*b"TREAD",*b"TREAT",*b"TRECK",
	*b"TREED",*b"TREEN",*b"TREES",*b"TREFA",*b"TREIF",*b"TREKS",*b"TREMA",*b"TREMS",*b"TREND",*b"TRESS",
	*b"TREST",*b"TRETS",*b"TREWS",*b"TREYF",*b"TREYS",*b"TRIAC",*b"TRIAD",*b"TRIAL",*b"TRIBE",*b"TRICE",
	*b"TRICK",*b"TRIDE",*b"TRIED",*b"TRIER",*b"TRIES",*b"TRIFF",*b"TRIGO",*b"TRIGS",*b"TRIKE",*b"TRILD",
	*b"TRILL",*b"TRIMS",*b"TRINE",*b"TRINS",*b"TRIOL",*b"TRIOR",*b"TRIOS",*b"TRIPE",*b"TRIPS",*b"TRIPY",
	*b"TRIST",*b"TRITE",*b"TROAD",*b"TROAK",*b"TROAT",*b"TROCK",*b"TRODE",*b"TRODS",*b"TROGS",*b"TROIS",
	*b"TROKE",*b"TROLL",*b"TROMP",*b"TRONA",*b"TRONC",*b"TRONE",*b"TRONK",*b"TRONS",*b"TROOP",*b"TROOZ",
	*b"TROPE",*b"TROTH",*b"TROTS",*b"TROUT",*b"TROVE",*b"TROWS",*b"TROYS",*b"TRUCE",*b"TRUCK",*b"TRUED",
	*b"TRUER",*b"TRUES",*b"TRUGO",*b"TRUGS",*b"TRULL",*b"TRULY",*b"TRUMP",*b"TRUNK",*b"TRUSS",*b"TRUST",
	*b"TRUTH",*b"TRYER",*b"TRYKE",*b"TRYMA",*b"TRYPS",*b"TRYST",*b"TSADE",*b"TSADI",*b"TSARS",*b"TSKED",
	*b"TSUBA",*b"TSUBO",*b"TUANS",*b"TUART",*b"TUATH",*b"TUBAE",*b"TUBAL",*b"TUBAR",*b"TUBAS",*b"TUBBY",
	*b"TUBED",*b"TUBER",*b"TUBES",*b"TUCKS",*b"TUFAS",*b"TUFFE",*b"TUFFS",*b"TUFTS",*b"TUFTY",*b"TUGRA",
	*b"TUILE",*b"TUINA",*b"TUISM",*b"TUKTU",*b"TULES",*b"TULIP",*b"TULLE",*b"TULPA",*b"TULSI",*b"TUMID",
	*b"TUMMY",*b"TUMOR",*b"TUMPS",*b"TUMPY",*b"TUNAS",*b"TUNDS",*b"TUNED",*b"TUNER",*b"TUNES",*b"TUNGS",
	*b"TUNIC",*b"TUNNY",*b"TUPEK",*b"TUPIK",*b"TUPLE",*b"TUQUE",*b"TURBO",*b"TURDS",*b"TURFS",*b"TURFY",
	*b"TURKS",*b"TURME",*b"TURMS",*b"TURNS",*b"TURNT",*b"TURPS",*b"TURRS",*b"TUSHY",*b"TUSKS",*b"TUSKY",
	*b"TUTEE",*b"TUTOR",*b"TUTTI",*b"TUTTY",*b"TUTUS",*b"TUXES",*b"TUYER",*b"TWAES",*b"TWAIN",*b"TWALS",
	*b"TWANG",*b"TWANK",*b"TWATS",*b"TWAYS",*b"TWEAK",*b"TWEED",*b"TWEEL",*b"TWEEN",*b"TWEEP",*b"TWEER",
	*b"TWEET",*b"TWERK",*b"TWERP",*b"TWICE",*b"TWIER",*b"TWIGS",*b"TWILL",*b"TWILT",*b"TWINE",*b"TWINK",
	*b"TWINS",*b"TWINY",*b"TWIRE",*b"TWIRL",*b"TWIRP",*b"TWIST",*b"TWITE",*b"TWITS",*b"TWIXT",*b"TWOER",
	*b"TWYER",*b"TYEES",*b"TYERS",*b"TYING",*b"TYIYN",*b"TYKES",*b"TYLER",*b"TYMPS",*b"TYNDE",*b"TYNED",
	*b"TYNES",*b"TYPAL",*b"TYPED",*b"TYPES",*b"TYPEY",*b"TYPIC",*b"TYPOS",*b"TYPPS",*b"TYPTO",*b"TYRAN",
	*b"TYRED",*b"TYRES",*b"TYROS",*b"TYTHE",*b"TZARS",*b"UDALS",*b"UDDER",*b"UDONS",*b"UGALI",*b"UGGED",
	*b"UHLAN",*b"UHURU",*b"UKASE",*b"ULAMA",*b"ULANS",*b"ULCER",*b"ULEMA",*b"ULMIN",*b"ULNAD",*b"ULNAE",
	*b"ULNAR",*b"ULNAS",*b"ULPAN",*b"ULTRA",*b"ULVAS",*b"ULYIE",*b"ULZIE",*b"UMAMI",*b"UMBEL",*b"UMBER",
	*b"UMBLE",*b"UMBOS",*b"UMBRA",*b"UMBRE",*b"UMIAC",*b"UMIAK",*b"UMIAQ",*b"UMMAH",*b"UMMAS",*b"UMMED",
	*b"UMPED",*b"UMPHS",*b"UMPIE",*b"UMPTY",*b"UMRAH",*b"UMRAS",*b"UNAIS",*b"UNAPT",*b"UNARM",*b"UNARY",
	*b"UNAUS",*b"UNBAG",*b"UNBAN",*b"UNBAR",*b"UNBED",*b"UNBID",*b"UNBOX",*b"UNCAP",*b"UNCES",*b"UNCIA",
	*b"UNCLE",*b"UNCOS",*b"UNCOY",*b"UNCUS",*b"UNCUT",*b"UNDAM",*b"UNDEE",*b"UNDER",*b"UNDID",*b"UNDOS",
	*b"UNDUE",*b"UNDUG",*b"UNETH",*b"UNFED",*b"UNFIT",*b"UNFIX",*b"UNGAG",*b"UNGET",*b"UNGOD",*b"UNGOT",
	*b"UNGUM",*b"UNHAT",*b"UNHIP",*b"UNICA",*b"UNIFY",*b"UNION",*b"UNITE",*b"UNITS",*b"UNITY",*b"UNJAM",
	*b"UNKED",*b"UNKET",*b"UNKID",*b"UNLAW",*b"UNLAY",*b"UNLED",*b"UNLET",*b"UNLID",*b"UNLIT",*b"UNMAN",
	*b"UNMET",*b"UNMEW",*b"UNMIX",*b"UNPAY",*b"UNPEG",*b"UNPEN",*b"UNPIN",*b"UNRED",*b"UNRID",*b"UNRIG",
	*b"UNRIP",*b"UNSAW",*b"UNSAY",*b"UNSEE",*b"UNSET",*b"UNSEW",*b"UNSEX",*b"UNSOD",*b"UNTAX",*b"UNTIE",
	*b"UNTIL",*b"UNTIN",*b"UNWED",*b"UNWET",*b"UNWIT",*b"UNWON",*b"UNZIP",*b"UPBOW",*b"UPBYE",*b"UPDOS",
	*b"UPDRY",*b"UPEND",*b"UPJET",*b"UPLAY",*b"UPLED",*b"UPLIT",*b"UPPED",*b"UPPER",*b"UPRAN",*b"UPRUN",
	*b"UPSEE",*b"UPSET",*b"UPSEY",*b"UPTAK",*b"UPTER",*b"UPTIE",*b"URAEI",*b"URALI",*b"URAOS",*b"URARE",
	*b"URARI",*b"URASE",*b"URATE",*b"URBAN",*b"URBEX",*b"URBIA",*b"URDEE",*b"UREAL",*b"UREAS",*b"UREDO",
	*b"UREIC",*b"URENA",*b"URENT",*b"URGED",*b"URGER",*b"URGES",*b"URIAL",*b"URINE",*b"URITE",*b"URMAN",
	*b"URNAL",*b"URNED",*b"URPED",*b"URSAE",*b"URSID",*b"URSON",*b"URUBU",*b"URVAS",*b"USAGE",*b"USERS",
	*b"USHER",*b"USING",*b"USNEA",*b"USQUE",*b"USUAL",*b"USURE",*b"USURP",*b"USURY",*b"UTERI",*b"UTILE",
	*b"UTTER",*b"UVEAL",*b"UVEAS",*b"UVULA",*b"VACUA",*b"VADED",*b"VADES",*b"VAGAL",*b"VAGUE",*b"VAGUS",
	*b"VAILS",*b"VAIRE",*b"VAIRS",*b"VAIRY",*b"VAKAS",*b"VAKIL",*b"VALES",*b"VALET",*b"VALID",*b"VALIS",
	*b"VALOR",*b"VALSE",*b"VALUE",*b"VALVE",*b"VAMPS",*b"VAMPY",*b"VANDA",*b"VANED",*b"VANES",*b"VANGS",
	*b"VANTS",*b"VAPED",*b"VAPER",*b"VAPES",*b"VAPID",*b"VAPOR",*b"VARAN",*b"VARAS",*b"VARDY",*b"VAREC",
	*b"VARES",*b"VARIA",*b"VARIX",*b"VARNA",*b"VARUS",*b"VARVE",*b"VASAL",*b"VASES",*b"VASTS",*b"VASTY",
	*b"VATIC",*b"VATUS",*b"VAUCH",*b"VAULT",*b"VAUNT",*b"VAUTE",*b"VAUTS",*b"VAWTE",*b"VAXES",*b"VEALE",
	*b"VEALS",*b"VEALY",*b"VEENA",*b"VEEPS",*b"VEERS",*b"VEERY",*b"VEGAN",*b"VEGAS",*b"VEGES",*b"VEGIE",
	*b"VEGOS",*b"VEHME",*b"VEILS",*b"VEILY",*b"VEINS",*b"VEINY",*b"VELAR",*b"VELDS",*b"VELDT",*b"VELES",
	*b"VELLS",*b"VELUM",*b"VENAE",*b"VENAL",*b"VENDS",*b"VENDU",*b"VENEY",*b"VENGE",*b"VENIN",*b"VENOM",
	*b"VENTS",*b"VENUE",*b"VENUS",*b"VERBS",*b"VERGE",*b"VERRA",*b"VERRY",*b"VERSE",*b"VERSO",*b"VERST",
	*b"VERTS",*b"VERTU",*b"VERVE",*b"VESPA",*b"VESTA",*b"VESTS",*b"VETCH",*b"VEXED",*b"VEXER",*b"VEXES",
	*b"VEXIL",*b"VEZIR",*b"VIALS",*b"VIAND",*b"VIBES",*b"VIBEX",*b"VIBEY",*b"VICAR",*b"VICED",*b"VICES",
	*b"VICHY",*b"VIDEO",*b"VIERS",*b"VIEWS",*b"VIEWY",*b"VIFDA",*b"VIFFS",*b"VIGAS",*b"VIGIA",*b"VIGIL",
	*b"VIGOR",*b"VILDE",*b"VILER",*b"VILLA",*b"VILLI",*b"VILLS",*b"VIMEN",*b"VINAL",*b"VINAS",*b"VINCA",
	*b"VINED",*b"VINER",*b"VINES",*b"VINEW",*b"VINIC",*b"VINOS",*b"VINTS",*b"VINYL",*b"VIOLA",*b"VIOLD",
	*b"VIOLS",*b"VIPER",*b"VIRAL",*b"VIRED",*b"VIREO",*b"VIRES",*b"VIRGA",*b"VIRGE",*b"VIRID",*b"VIRLS",
	*b"VIRTU",*b"VIRUS",*b"VISAS",*b"VISED",*b"VISES",*b"VISIE",*b"VISIT",*b"VISNE",*b"VISON",*b"VISOR",
	*b"VISTA",*b"VISTO",*b"VITAE",*b"VITAL",*b"VITAS",*b"VITEX",*b"VITRO",*b"VITTA",*b"VIVAS",*b"VIVAT",
	*b"VIVDA",*b"VIVER",*b"VIVES",*b"VIVID",*b"VIXEN",*b"VIZIR",*b"VIZOR",*b"VLEIS",*b"VLIES",*b"VLOGS",
	*b"VOARS",*b"VOCAB",*b"VOCAL",*b"VOCES",*b"VODDY",*b"VODKA",*b"VODOU",*b"VODUN",*b"VOEMA",*b"VOGIE",
	*b"VOGUE",*b"VOICE",*b"VOIDS",*b"VOILA",*b"VOILE",*b"VOIPS",*b"VOLAE",*b"VOLAR",*b"VOLED",*b"VOLES",
	*b"VOLET",*b"VOLKS",*b"VOLTA",*b"VOLTE",*b"VOLTI",*b"VOLTS",*b"VOLVA",*b"VOLVE",*b"VOMER",*b"VOMIT",
	*b"VOTED",*b"VOTER",*b"VOTES",*b"VOUCH",*b"VOUGE",*b"VOULU",*b"VOWED",*b"VOWEL",*b"VOWER",*b"VOXEL",
	*b"VOZHD",*b"VRAIC",*b"VRILS",*b"VROOM",*b"VROUS",*b"VROUW",*b"VROWS",*b"VUGGS",*b"VUGGY",*b"VUGHS",
	*b"VUGHY",*b"VULGO",*b"VULNS",*b"VULVA",*b"VUTTY",*b"VYING",*b"WAACS",*b"WACKE",*b"WACKO",*b"WACKS",
	*b"WACKY",*b"WADDS",*b"WADDY",*b"WADED",*b"WADER",*b"WADES",*b"WADGE",*b"WADIS",*b"WADTS",*b"WAFER",
	*b"WAFFS",*b"WAFTS",*b"WAGED",*b"WAGER",*b"WAGES",*b"WAGGA",*b"WAGON",*b"WAGYU",*b"WAHOO",*b"WAIDE",
	*b"WAIFS",*b"WAIFT",*b"WAILS",*b"WAINS",*b"WAIRS",*b"WAIST",*b"WAITE",*b"WAITS",*b"WAIVE",*b"WAKAS",
	*b"WAKED",*b"WAKEN",*b"WAKER",*b"WAKES",*b"WAKFS",*b"WALDO",*b"WALDS",*b"WALED",*b"WALER",*b"WALES",
	*b"WALIE",*b"WALIS",*b"WALKS",*b"WALLA",*b"WALLS",*b"WALLY",*b"WALTY",*b"WALTZ",*b"WAMED",*b"WAMES",
	*b"WAMUS",*b"WANDS",*b"WANED",*b"WANES",*b"WANEY",*b"WANGS",*b"WANKS",*b"WANKY",*b"WANLE",*b"WANLY",
	*b"WANNA",*b"WANTS",*b"WANTY",*b"WANZE",*b"WAQFS",*b"WARBS",*b"WARBY",*b"WARDS",*b"WARED",*b"WARES",
	*b"WAREZ",*b"WARKS",*b"WARMS",*b"WARNS",*b"WARPS",*b"WARRE",*b"WARST",*b"WARTS",*b"WARTY",*b"WASES",
	*b"WASHY",*b"WASMS",*b"WASPS",*b"WASPY",*b"WASTE",*b"WASTS",*b"WATAP",*b"WATCH",*b"WATER",*b"WATTS",
	*b"WAUFF",*b"WAUGH",*b"WAUKS",*b"WAULK",*b"WAULS",*b"WAURS",*b"WAVED",*b"WAVER",*b"WAVES",*b"WAVEY",
	*b"WAWAS",*b"WAWES",*b"WAWLS",*b"WAXED",*b"WAXEN",*b"WAXER",*b"WAXES",*b"WAYED",*b"WAZIR",*b"WAZOO",
	*b"WEALD",*b"WEALS",*b"WEAMB",*b"WEANS",*b"WEARS",*b"WEARY",*b"WEAVE",*b"WEBBY",*b"WEBER",*b"WECHT",
	*b"WEDEL",*b"WEDGE",*b"WEDGY",*b"WEEDS",*b"WEEDY",*b"WEEKE",*b"WEEKS",*b"WEELS",*b"WEEMS",*b"WEENS",
	*b"WEENY",*b"WEEPS",*b"WEEPY",*b"WEEST",*b"WEETE",*b"WEETS",*b"WEFTE",*b"WEFTS",*b"WEIDS",*b"WEIGH",
	*b"WEILS",*b"WEIRD",*b"WEIRS",*b"WEISE",*b"WEIZE",*b"WEKAS",*b"WELCH",*b"WELDS",*b"WELKE",*b"WELKS",
	*b"WELKT",*b"WELLS",*b"WELLY",*b"WELSH",*b"WELTS",*b"WEMBS",*b"WENCH",*b"WENDS",*b"WENGE",*b"WENNY",
	*b"WENTS",*b"WEROS",*b"WERSH",*b"WESTS",*b"WETAS",*b"WETLY",*b"WEXED",*b"WEXES",*b"WHACK",*b"WHALE",
	*b"WHAMO",*b"WHAMS",*b"WHANG",*b"WHAPS",*b"WHARE",*b"WHARF",*b"WHATA",*b"WHATS",*b"WHAUP",*b"WHAUR",
	*b"WHEAL",*b"WHEAR",*b"WHEAT",*b"WHEEL",*b"WHEEN",*b"WHEEP",*b"WHEFT",*b"WHELK",*b"WHELM",*b"WHELP",
	*b"WHENS",*b"WHERE",*b"WHETS",*b"WHEWS",*b"WHEYS",*b"WHICH",*b"WHIDS",*b"WHIFF",*b"WHIFT",*b"WHIGS",
	*b"WHILE",*b"WHILK",*b"WHIMS",*b"WHINE",*b"WHINS",*b"WHINY",*b"WHIOS",*b"WHIPS",*b"WHIPT",*b"WHIRL",
	*b"WHIRR",*b"WHIRS",*b"WHISH",*b"WHISK",*b"WHISS",*b"WHIST",*b"WHITE",*b"WHITS",*b"WHITY",*b"WHIZZ",
	*b"WHOLE",*b"WHOMP",*b"WHOOF",*b"WHOOP",*b"WHOOT",*b"WHOPS",*b"WHORE",*b"WHORL",*b"WHORT",*b"WHOSE",
	*b"WHOSO",*b"WHOWS",*b"WHUMP",*b"WHUPS",*b"WHYDA",*b"WICCA",*b"WICKS",*b"WICKY",*b"WIDDY",*b"WIDEN",
	*b"WIDER",*b"WIDES",*b"WIDOW",*b"WIDTH",*b"WIELD",*b"WIELS",*b"WIFED",*b"WIFES",*b"WIFEY",*b"WIFIE",
	*b"WIFTY",*b"WIGAN",*b"WIGGA",*b"WIGGY",*b"WIGHT",*b"WIKIS",*b"WILCO",*b"WILDS",*b"WILED",*b"WILES",
	*b"WILGA",*b"WILIS",*b"WILJA",*b"WILLS",*b"WILLY",*b"WILTS",*b"WIMPS",*b"WIMPY",*b"WINCE",*b"WINCH",
	*b"WINDS",*b"WINDY",*b"WINED",*b"WINES",*b"WINEY",*b"WINGE",*b"WINGS",*b"WINGY",*b"WINKS",*b"WINNA",
	*b"WINNS",*b"WINOS",*b"WINZE",*b"WIPED",*b"WIPER",*b"WIPES",*b"WIRED",*b"WIRER",*b"WIRES",*b"WIRRA",
	*b"WISED",*b"WISER",*b"WISES",*b"WISHA",*b"WISHT",*b"WISPS",*b"WISPY",*b"WISTS",*b"WITAN",*b"WITCH",
	*b"WITED",*b"WITES",*b"WITHE",*b"WITHS",*b"WITHY",*b"WITTY",*b"WIVED",*b"WIVER",*b"WIVES",*b"WIZEN",
	*b"WIZES",*b"WOADS",*b"WOALD",*b"WOCKS",*b"WODGE",*b"WOFUL",*b"WOJUS",*b"WOKEN",*b"WOKER",*b"WOKKA",
	*b"WOLDS",*b"WOLFS",*b"WOLLY",*b"WOLVE",*b"WOMAN",*b"WOMBS",*b"WOMBY",*b"WOMEN",*b"WOMYN",*b"WONGA",
	*b"WONGI",*b"WONKS",*b"WONKY",*b"WONTS",*b"WOODS",*b"WOODY",*b"WOOED",*b"WOOER",*b"WOOFS",*b"WOOFY",
	*b"WOOLD",*b"WOOLS",*b"WOOLY",*b"WOONS",*b"WOOPS",*b"WOOPY",*b"WOOSE",*b"WOOSH",*b"WOOTZ",*b"WOOZY",
	*b"WORDS",*b"WORDY",*b"WORKS",*b"WORLD",*b"WORMS",*b"WORMY",*b"WORRY",*b"WORSE",*b"WORST",*b"WORTH",
	*b"WORTS",*b"WOULD",*b"WOUND",*b"WOVEN",*b"WOWED",*b"WOWEE",*b"WOXEN",*b"WRACK",*b"WRANG",*b"WRAPS",
	*b"WRAPT",*b"WRAST",*b"WRATE",*b"WRATH",*b"WRAWL",*b"WREAK",*b"WRECK",*b"WRENS",*b"WREST",*b"WRICK",
	*b"WRIED",*b"WRIER",*b"WRIES",*b"WRING",*b"WRIST",*b"WRITE",*b"WRITS",*b"WROKE",*b"WRONG",*b"WROOT",
	*b"WROTE",*b"WROTH",*b"WRUNG",*b"WRYER",*b"WRYLY",*b"WUDDY",*b"WUDUS",*b"WULLS",*b"WURST",*b"WUSES",
	*b"WUSHU",*b"WUSSY",*b"WUXIA",*b"WYLED",*b"WYLES",*b"WYNDS",*b"WYNNS",*b"WYTED",*b"WYTES",*b"XEBEC",
	*b"XENIA",*b"XENIC",*b"XENON",*b"XERIC",*b"XEROX",*b"XERUS",*b"XOANA",*b"XRAYS",*b"XYLAN",*b"XYLEM",
	*b"XYLIC",*b"XYLOL",*b"XYLYL",*b"XYSTI",*b"XYSTS",*b"YAARS",*b"YABAS",*b"YABBA",*b"YABBY",*b"YACCA",
	*b"YACHT",*b"YACKA",*b"YACKS",*b"YAFFS",*b"YAGER",*b"YAGES",*b"YAGIS",*b"YAHOO",*b"YAIRD",*b"YAKKA",
	*b"YAKOW",*b"YALES",*b"YAMEN",*b"YAMPY",*b"YAMUN",*b"YANGS",*b"YANKS",*b"YAPOK",*b"YAPON",*b"YAPPS",
	*b"YAPPY",*b"YARAK",*b"YARCO",*b"YARDS",*b"YARER",*b"YARFA",*b"YARKS",*b"YARNS",*b"YARRS",*b"YARTA",
	*b"YARTO",*b"YATES",*b"YAUDS",*b"YAULD",*b"YAUPS",*b"YAWED",*b"YAWEY",*b"YAWLS",*b"YAWNS",*b"YAWNY",
	*b"YAWPS",*b"YBORE",*b"YCLAD",*b"YCLED",*b"YCOND",*b"YDRAD",*b"YDRED",*b"YEADS",*b"YEAHS",*b"YEALM",
	*b"YEANS",*b"YEARD",*b"YEARN",*b"YEARS",*b"YEAST",*b"YECCH",*b"YECHS",*b"YECHY",*b"YEDES",*b"YEEDS",
	*b"YEESH",*b"YEGGS",*b"YELKS",*b"YELLS",*b"YELMS",*b"YELPS",*b"YELTS",*b"YENTA",*b"YENTE",*b"YERBA",
	*b"YERDS",*b"YERKS",*b"YESES",*b"YESKS",*b"YESTS",*b"YESTY",*b"YETIS",*b"YETTS",*b"YEUKS",*b"YEUKY",
	*b"YEVEN",*b"YEVES",*b"YEWEN",*b"YEXED",*b"YEXES",*b"YFERE",*b"YIELD",*b"YIKED",*b"YIKES",*b"YILLS",
	*b"YINCE",*b"YIPES",*b"YIPPY",*b"YIRDS",*b"YIRKS",*b"YIRRS",*b"YIRTH",*b"YITES",*b"YITIE",*b"YLEMS",
	*b"YLIKE",*b"YLKES",*b"YMOLT",*b"YMPES",*b"YOBBO",*b"YOBBY",*b"YOCKS",*b"YODEL",*b"YODHS",*b"YODLE",
	*b"YOGAS",*b"YOGEE",*b"YOGHS",*b"YOGIC",*b"YOGIN",*b"YOGIS",*b"YOICK",*b"YOJAN",*b"YOKED",*b"YOKEL",
	*b"YOKER",*b"YOKES",*b"YOKUL",*b"YOLKS",*b"YOLKY",*b"YOMIM",*b"YOMPS",*b"YONIC",*b"YONIS",*b"YONKS",
	*b"YOOFS",*b"YOOPS",*b"YORES",*b"YORKS",*b"YORPS",*b"YOUKS",*b"YOUNG",*b"YOURN",*b"YOURS",*b"YOURT",
	*b"YOUSE",*b"YOUTH",*b"YOWED",*b"YOWES",*b"YOWIE",*b"YOWLS",*b"YOWZA",*b"YRAPT",*b"YRENT",*b"YRIVD",
	*b"YRNEH",*b"YSAME",*b"YTOST",*b"YUANS",*b"YUCAS",*b"YUCCA",*b"YUCCH",*b"YUCKO",*b"YUCKS",*b"YUCKY",
	*b"YUFTS",*b"YUGAS",*b"YUKED",*b"YUKES",*b"YUKKY",*b"YUKOS",*b"YULAN",*b"YULES",*b"YUMMO",*b"YUMMY",
	*b"YUMPS",*b"YUPON",*b"YUPPY",*b"YURTA",*b"YURTS",*b"YUZUS",*b"ZABRA",*b"ZACKS",*b"ZAIDA",*b"ZAIDY",
	*b"ZAIRE",*b"ZAKAT",*b"ZAMAN",*b"ZAMBO",*b"ZAMIA",*b"ZANJA",*b"ZANTE",*b"ZANZA",*b"ZANZE",*b"ZAPPY",
	*b"ZARFS",*b"ZARIS",*b"ZATIS",*b"ZAXES",*b"ZAYIN",*b"ZAZEN",*b"ZEALS",*b"ZEBEC",*b"ZEBRA",*b"ZEBUB",
	*b"ZEBUS",*b"ZEDAS",*b"ZEINS",*b"ZENDO",*b"ZERDA",*b"ZERKS",*b"ZEROS",*b"ZESTS",*b"ZESTY",*b"ZETAS",
	*b"ZEXES",*b"ZEZES",*b"ZHOMO",*b"ZIBET",*b"ZIFFS",*b"ZIGAN",*b"ZILAS",*b"ZILCH",*b"ZILLA",*b"ZILLS",
	*b"ZIMBI",*b"ZIMBS",*b"ZINCO",*b"ZINCS",*b"ZINCY",*b"ZINEB",*b"ZINES",*b"ZINGS",*b"ZINGY",*b"ZINKE",
	*b"ZINKY",*b"ZIPPO",*b"ZIPPY",*b"ZIRAM",*b"ZITIS",*b"ZIZEL",*b"ZIZIT",*b"ZLOTE",*b"ZLOTY",*b"ZOAEA",
	*b"ZOBOS",*b"ZOBUS",*b"ZOCCO",*b"ZOEAE",*b"ZOEAL",*b"ZOEAS",*b"ZOISM",*b"ZOIST",*b"ZOMBI",*b"ZONAE",
	*b"ZONAL",*b"ZONDA",*b"ZONED",*b"ZONER",*b"ZONES",*b"ZONKS",*b"ZOOEA",*b"ZOOEY",*b"ZOOID",*b"ZOOKS",
	*b"ZOOMS",*b"ZOONS",*b"ZOOTY",*b"ZOPPA",*b"ZOPPO",*b"ZORIL",*b"ZORIS",*b"ZORRO",*b"ZOUKS",*b"ZOWEE",
	*b"ZOWIE",*b"ZULUS",*b"ZUPAN",*b"ZUPAS",*b"ZUPPA",*b"ZURFS",*b"ZUZIM",*b"ZYGAL",*b"ZYGON",*b"ZYMES",
	*b"ZYMIC"
];

// i cant be bothered to figure out how to do this correctly
#[macro_export]
macro_rules! clues {
	(_ _ _ _ _) => {WordleClue::new(0)};
	(_ _ _ _ Y) => {WordleClue::new(1)};
	(_ _ _ _ G) => {WordleClue::new(2)};
	(_ _ _ Y _) => {WordleClue::new(3)};
	(_ _ _ Y Y) => {WordleClue::new(4)};
	(_ _ _ Y G) => {WordleClue::new(5)};
	(_ _ _ G _) => {WordleClue::new(6)};
	(_ _ _ G Y) => {WordleClue::new(7)};
	(_ _ _ G G) => {WordleClue::new(8)};
	(_ _ Y _ _) => {WordleClue::new(9)};
	(_ _ Y _ Y) => {WordleClue::new(10)};
	(_ _ Y _ G) => {WordleClue::new(11)};
	(_ _ Y Y _) => {WordleClue::new(12)};
	(_ _ Y Y Y) => {WordleClue::new(13)};
	(_ _ Y Y G) => {WordleClue::new(14)};
	(_ _ Y G _) => {WordleClue::new(15)};
	(_ _ Y G Y) => {WordleClue::new(16)};
	(_ _ Y G G) => {WordleClue::new(17)};
	(_ _ G _ _) => {WordleClue::new(18)};
	(_ _ G _ Y) => {WordleClue::new(19)};
	(_ _ G _ G) => {WordleClue::new(20)};
	(_ _ G Y _) => {WordleClue::new(21)};
	(_ _ G Y Y) => {WordleClue::new(22)};
	(_ _ G Y G) => {WordleClue::new(23)};
	(_ _ G G _) => {WordleClue::new(24)};
	(_ _ G G Y) => {WordleClue::new(25)};
	(_ _ G G G) => {WordleClue::new(26)};
	(_ Y _ _ _) => {WordleClue::new(27)};
	(_ Y _ _ Y) => {WordleClue::new(28)};
	(_ Y _ _ G) => {WordleClue::new(29)};
	(_ Y _ Y _) => {WordleClue::new(30)};
	(_ Y _ Y Y) => {WordleClue::new(31)};
	(_ Y _ Y G) => {WordleClue::new(32)};
	(_ Y _ G _) => {WordleClue::new(33)};
	(_ Y _ G Y) => {WordleClue::new(34)};
	(_ Y _ G G) => {WordleClue::new(35)};
	(_ Y Y _ _) => {WordleClue::new(36)};
	(_ Y Y _ Y) => {WordleClue::new(37)};
	(_ Y Y _ G) => {WordleClue::new(38)};
	(_ Y Y Y _) => {WordleClue::new(39)};
	(_ Y Y Y Y) => {WordleClue::new(40)};
	(_ Y Y Y G) => {WordleClue::new(41)};
	(_ Y Y G _) => {WordleClue::new(42)};
	(_ Y Y G Y) => {WordleClue::new(43)};
	(_ Y Y G G) => {WordleClue::new(44)};
	(_ Y G _ _) => {WordleClue::new(45)};
	(_ Y G _ Y) => {WordleClue::new(46)};
	(_ Y G _ G) => {WordleClue::new(47)};
	(_ Y G Y _) => {WordleClue::new(48)};
	(_ Y G Y Y) => {WordleClue::new(49)};
	(_ Y G Y G) => {WordleClue::new(50)};
	(_ Y G G _) => {WordleClue::new(51)};
	(_ Y G G Y) => {WordleClue::new(52)};
	(_ Y G G G) => {WordleClue::new(53)};
	(_ G _ _ _) => {WordleClue::new(54)};
	(_ G _ _ Y) => {WordleClue::new(55)};
	(_ G _ _ G) => {WordleClue::new(56)};
	(_ G _ Y _) => {WordleClue::new(57)};
	(_ G _ Y Y) => {WordleClue::new(58)};
	(_ G _ Y G) => {WordleClue::new(59)};
	(_ G _ G _) => {WordleClue::new(60)};
	(_ G _ G Y) => {WordleClue::new(61)};
	(_ G _ G G) => {WordleClue::new(62)};
	(_ G Y _ _) => {WordleClue::new(63)};
	(_ G Y _ Y) => {WordleClue::new(64)};
	(_ G Y _ G) => {WordleClue::new(65)};
	(_ G Y Y _) => {WordleClue::new(66)};
	(_ G Y Y Y) => {WordleClue::new(67)};
	(_ G Y Y G) => {WordleClue::new(68)};
	(_ G Y G _) => {WordleClue::new(69)};
	(_ G Y G Y) => {WordleClue::new(70)};
	(_ G Y G G) => {WordleClue::new(71)};
	(_ G G _ _) => {WordleClue::new(72)};
	(_ G G _ Y) => {WordleClue::new(73)};
	(_ G G _ G) => {WordleClue::new(74)};
	(_ G G Y _) => {WordleClue::new(75)};
	(_ G G Y Y) => {WordleClue::new(76)};
	(_ G G Y G) => {WordleClue::new(77)};
	(_ G G G _) => {WordleClue::new(78)};
	(_ G G G Y) => {WordleClue::new(79)};
	(_ G G G G) => {WordleClue::new(80)};
	(Y _ _ _ _) => {WordleClue::new(81)};
	(Y _ _ _ Y) => {WordleClue::new(82)};
	(Y _ _ _ G) => {WordleClue::new(83)};
	(Y _ _ Y _) => {WordleClue::new(84)};
	(Y _ _ Y Y) => {WordleClue::new(85)};
	(Y _ _ Y G) => {WordleClue::new(86)};
	(Y _ _ G _) => {WordleClue::new(87)};
	(Y _ _ G Y) => {WordleClue::new(88)};
	(Y _ _ G G) => {WordleClue::new(89)};
	(Y _ Y _ _) => {WordleClue::new(90)};
	(Y _ Y _ Y) => {WordleClue::new(91)};
	(Y _ Y _ G) => {WordleClue::new(92)};
	(Y _ Y Y _) => {WordleClue::new(93)};
	(Y _ Y Y Y) => {WordleClue::new(94)};
	(Y _ Y Y G) => {WordleClue::new(95)};
	(Y _ Y G _) => {WordleClue::new(96)};
	(Y _ Y G Y) => {WordleClue::new(97)};
	(Y _ Y G G) => {WordleClue::new(98)};
	(Y _ G _ _) => {WordleClue::new(99)};
	(Y _ G _ Y) => {WordleClue::new(100)};
	(Y _ G _ G) => {WordleClue::new(101)};
	(Y _ G Y _) => {WordleClue::new(102)};
	(Y _ G Y Y) => {WordleClue::new(103)};
	(Y _ G Y G) => {WordleClue::new(104)};
	(Y _ G G _) => {WordleClue::new(105)};
	(Y _ G G Y) => {WordleClue::new(106)};
	(Y _ G G G) => {WordleClue::new(107)};
	(Y Y _ _ _) => {WordleClue::new(108)};
	(Y Y _ _ Y) => {WordleClue::new(109)};
	(Y Y _ _ G) => {WordleClue::new(110)};	
	(Y Y _ Y _) => {WordleClue::new(111)};
	(Y Y _ Y Y) => {WordleClue::new(112)};
	(Y Y _ Y G) => {WordleClue::new(113)};
	(Y Y _ G _) => {WordleClue::new(114)};
	(Y Y _ G Y) => {WordleClue::new(115)};
	(Y Y _ G G) => {WordleClue::new(116)};
	(Y Y Y _ _) => {WordleClue::new(117)};
	(Y Y Y _ Y) => {WordleClue::new(118)};
	(Y Y Y _ G) => {WordleClue::new(119)};
	(Y Y Y Y _) => {WordleClue::new(120)};
	(Y Y Y Y Y) => {WordleClue::new(121)};
	(Y Y Y Y G) => {WordleClue::new(122)};
	(Y Y Y G _) => {WordleClue::new(123)};
	(Y Y Y G Y) => {WordleClue::new(124)};
	(Y Y Y G G) => {WordleClue::new(125)};
	(Y Y G _ _) => {WordleClue::new(126)};
	(Y Y G _ Y) => {WordleClue::new(127)};
	(Y Y G _ G) => {WordleClue::new(128)};
	(Y Y G Y _) => {WordleClue::new(129)};
	(Y Y G Y Y) => {WordleClue::new(130)};
	(Y Y G Y G) => {WordleClue::new(131)};
	(Y Y G G _) => {WordleClue::new(132)};
	(Y Y G G Y) => {WordleClue::new(133)};
	(Y Y G G G) => {WordleClue::new(134)};
	(Y G _ _ _) => {WordleClue::new(135)};
	(Y G _ _ Y) => {WordleClue::new(136)};
	(Y G _ _ G) => {WordleClue::new(137)};
	(Y G _ Y _) => {WordleClue::new(138)};
	(Y G _ Y Y) => {WordleClue::new(139)};
	(Y G _ Y G) => {WordleClue::new(140)};
	(Y G _ G _) => {WordleClue::new(141)};
	(Y G _ G Y) => {WordleClue::new(142)};
	(Y G _ G G) => {WordleClue::new(143)};
	(Y G Y _ _) => {WordleClue::new(144)};
	(Y G Y _ Y) => {WordleClue::new(145)};
	(Y G Y _ G) => {WordleClue::new(146)};
	(Y G Y Y _) => {WordleClue::new(147)};
	(Y G Y Y Y) => {WordleClue::new(148)};
	(Y G Y Y G) => {WordleClue::new(149)};
	(Y G Y G _) => {WordleClue::new(150)};
	(Y G Y G Y) => {WordleClue::new(151)};
	(Y G Y G G) => {WordleClue::new(152)};
	(Y G G _ _) => {WordleClue::new(153)};
	(Y G G _ Y) => {WordleClue::new(154)};
	(Y G G _ G) => {WordleClue::new(155)};
	(Y G G Y _) => {WordleClue::new(156)};
	(Y G G Y Y) => {WordleClue::new(157)};
	(Y G G Y G) => {WordleClue::new(158)};
	(Y G G G _) => {WordleClue::new(159)};
	(Y G G G Y) => {WordleClue::new(160)};
	(Y G G G G) => {WordleClue::new(161)};
	(G _ _ _ _) => {WordleClue::new(162)};
	(G _ _ _ Y) => {WordleClue::new(163)};
	(G _ _ _ G) => {WordleClue::new(164)};
	(G _ _ Y _) => {WordleClue::new(165)};
	(G _ _ Y Y) => {WordleClue::new(166)};
	(G _ _ Y G) => {WordleClue::new(167)};
	(G _ _ G _) => {WordleClue::new(168)};
	(G _ _ G Y) => {WordleClue::new(169)};
	(G _ _ G G) => {WordleClue::new(170)};
	(G _ Y _ _) => {WordleClue::new(171)};
	(G _ Y _ Y) => {WordleClue::new(172)};
	(G _ Y _ G) => {WordleClue::new(173)};
	(G _ Y Y _) => {WordleClue::new(174)};
	(G _ Y Y Y) => {WordleClue::new(175)};
	(G _ Y Y G) => {WordleClue::new(176)};
	(G _ Y G _) => {WordleClue::new(177)};
	(G _ Y G Y) => {WordleClue::new(178)};
	(G _ Y G G) => {WordleClue::new(179)};
	(G _ G _ _) => {WordleClue::new(180)};
	(G _ G _ Y) => {WordleClue::new(181)};
	(G _ G _ G) => {WordleClue::new(182)};
	(G _ G Y _) => {WordleClue::new(183)};
	(G _ G Y Y) => {WordleClue::new(184)};
	(G _ G Y G) => {WordleClue::new(185)};
	(G _ G G _) => {WordleClue::new(186)};
	(G _ G G Y) => {WordleClue::new(187)};
	(G _ G G G) => {WordleClue::new(188)};
	(G Y _ _ _) => {WordleClue::new(189)};
	(G Y _ _ Y) => {WordleClue::new(190)};
	(G Y _ _ G) => {WordleClue::new(191)};
	(G Y _ Y _) => {WordleClue::new(192)};
	(G Y _ Y Y) => {WordleClue::new(193)};
	(G Y _ Y G) => {WordleClue::new(194)};
	(G Y _ G _) => {WordleClue::new(195)};
	(G Y _ G Y) => {WordleClue::new(196)};
	(G Y _ G G) => {WordleClue::new(197)};
	(G Y Y _ _) => {WordleClue::new(198)};
	(G Y Y _ Y) => {WordleClue::new(199)};
	(G Y Y _ G) => {WordleClue::new(200)};
	(G Y Y Y _) => {WordleClue::new(201)};
	(G Y Y Y Y) => {WordleClue::new(202)};
	(G Y Y Y G) => {WordleClue::new(203)};
	(G Y Y G _) => {WordleClue::new(204)};
	(G Y Y G Y) => {WordleClue::new(205)};
	(G Y Y G G) => {WordleClue::new(206)};
	(G Y G _ _) => {WordleClue::new(207)};
	(G Y G _ Y) => {WordleClue::new(208)};
	(G Y G _ G) => {WordleClue::new(209)};
	(G Y G Y _) => {WordleClue::new(210)};
	(G Y G Y Y) => {WordleClue::new(211)};
	(G Y G Y G) => {WordleClue::new(212)};
	(G Y G G _) => {WordleClue::new(213)};
	(G Y G G Y) => {WordleClue::new(214)};
	(G Y G G G) => {WordleClue::new(215)};
	(G G _ _ _) => {WordleClue::new(216)};
	(G G _ _ Y) => {WordleClue::new(217)};
	(G G _ _ G) => {WordleClue::new(218)};
	(G G _ Y _) => {WordleClue::new(219)};
	(G G _ Y Y) => {WordleClue::new(220)};
	(G G _ Y G) => {WordleClue::new(221)};
	(G G _ G _) => {WordleClue::new(222)};
	(G G _ G Y) => {WordleClue::new(223)};
	(G G _ G G) => {WordleClue::new(224)};
	(G G Y _ _) => {WordleClue::new(225)};
	(G G Y _ Y) => {WordleClue::new(226)};
	(G G Y _ G) => {WordleClue::new(227)};
	(G G Y Y _) => {WordleClue::new(228)};
	(G G Y Y Y) => {WordleClue::new(229)};
	(G G Y Y G) => {WordleClue::new(230)};
	(G G Y G _) => {WordleClue::new(231)};
	(G G Y G Y) => {WordleClue::new(232)};
	(G G Y G G) => {WordleClue::new(233)};
	(G G G _ _) => {WordleClue::new(234)};
	(G G G _ Y) => {WordleClue::new(235)};
	(G G G _ G) => {WordleClue::new(236)};
	(G G G Y _) => {WordleClue::new(237)};
	(G G G Y Y) => {WordleClue::new(238)};
	(G G G Y G) => {WordleClue::new(239)};
	(G G G G _) => {WordleClue::new(240)};
	(G G G G Y) => {WordleClue::new(241)};
	(G G G G G) => {WordleClue::new(242)};
}

impl WordleClue {
	pub const fn sum_of_base3_digits(self) -> u8 {
		match self {
			// if youre reading this ur a nerd and shouldnt complain about this absolute horror
			// TODO: check these cuz i used github copilot to write all this after 15 or so
			WordleClue(0) => 0, // 0t00000
			WordleClue(1) => 1, // 0t00001
			WordleClue(2) => 2, // 0t00002
			WordleClue(3) => 1, // 0t00010
			WordleClue(4) => 2, // 0t00011
			WordleClue(5) => 3, // 0t00012
			WordleClue(6) => 2, // 0t00020
			WordleClue(7) => 3, // 0t00021
			WordleClue(8) => 4, // 0t00022
			WordleClue(9) => 1, // 0t00100
			WordleClue(10) => 2, // 0t00101
			WordleClue(11) => 3, // 0t00102
			WordleClue(12) => 2, // 0t00110
			WordleClue(13) => 3, // 0t00111
			WordleClue(14) => 4, // 0t00112
			WordleClue(15) => 3, // 0t00120
			WordleClue(16) => 4, // 0t00121
			WordleClue(17) => 5, // 0t00122
			WordleClue(18) => 2, // 0t00200
			WordleClue(19) => 3, // 0t00201
			WordleClue(20) => 4, // 0t00202
			WordleClue(21) => 3, // 0t00210
			WordleClue(22) => 4, // 0t00211
			WordleClue(23) => 5, // 0t00212
			WordleClue(24) => 4, // 0t00220
			WordleClue(25) => 5, // 0t00221
			WordleClue(26) => 6, // 0t00222
			WordleClue(27) => 1, // 0t01000
			WordleClue(28) => 2, // 0t01001
			WordleClue(29) => 3, // 0t01002
			WordleClue(30) => 2, // 0t01010
			WordleClue(31) => 3, // 0t01011
			WordleClue(32) => 4, // 0t01012
			WordleClue(33) => 3, // 0t01020
			WordleClue(34) => 4, // 0t01021
			WordleClue(35) => 5, // 0t01022
			WordleClue(36) => 2, // 0t01100
			WordleClue(37) => 3, // 0t01101
			WordleClue(38) => 4, // 0t01102
			WordleClue(39) => 3, // 0t01110
			WordleClue(40) => 4, // 0t01111
			WordleClue(41) => 5, // 0t01112
			WordleClue(42) => 4, // 0t01120
			WordleClue(43) => 5, // 0t01121
			WordleClue(44) => 6, // 0t01122
			WordleClue(45) => 3, // 0t01200
			WordleClue(46) => 4, // 0t01201
			WordleClue(47) => 5, // 0t01202
			WordleClue(48) => 4, // 0t01210
			WordleClue(49) => 5, // 0t01211
			WordleClue(50) => 6, // 0t01212
			WordleClue(51) => 5, // 0t01220
			WordleClue(52) => 6, // 0t01221
			WordleClue(53) => 7, // 0t01222
			WordleClue(54) => 2, // 0t02000
			WordleClue(55) => 3, // 0t02001
			WordleClue(56) => 4, // 0t02002
			WordleClue(57) => 3, // 0t02010
			WordleClue(58) => 4, // 0t02011
			WordleClue(59) => 5, // 0t02012
			WordleClue(60) => 4, // 0t02020
			WordleClue(61) => 5, // 0t02021
			WordleClue(62) => 6, // 0t02022
			WordleClue(63) => 3, // 0t02100
			WordleClue(64) => 4, // 0t02101
			WordleClue(65) => 5, // 0t02102
			WordleClue(66) => 4, // 0t02110
			WordleClue(67) => 5, // 0t02111
			WordleClue(68) => 6, // 0t02112
			WordleClue(69) => 5, // 0t02120
			WordleClue(70) => 6, // 0t02121
			WordleClue(71) => 7, // 0t02122
			WordleClue(72) => 4, // 0t02200
			WordleClue(73) => 5, // 0t02201
			WordleClue(74) => 6, // 0t02202
			WordleClue(75) => 5, // 0t02210
			WordleClue(76) => 6, // 0t02211
			WordleClue(77) => 7, // 0t02212
			WordleClue(78) => 6, // 0t02220
			WordleClue(79) => 7, // 0t02221
			WordleClue(80) => 8, // 0t02222
			WordleClue(81) => 1, // 0t10000
			WordleClue(82) => 2, // 0t10001
			WordleClue(83) => 3, // 0t10002
			WordleClue(84) => 2, // 0t10010
			WordleClue(85) => 3, // 0t10011
			WordleClue(86) => 4, // 0t10012
			WordleClue(87) => 3, // 0t10020
			WordleClue(88) => 4, // 0t10021
			WordleClue(89) => 5, // 0t10022
			WordleClue(90) => 2, // 0t10100
			WordleClue(91) => 3, // 0t10101
			WordleClue(92) => 4, // 0t10102
			WordleClue(93) => 3, // 0t10110
			WordleClue(94) => 4, // 0t10111
			WordleClue(95) => 5, // 0t10112
			WordleClue(96) => 4, // 0t10120
			WordleClue(97) => 5, // 0t10121
			WordleClue(98) => 6, // 0t10122
			WordleClue(99) => 3, // 0t10200
			WordleClue(100) => 4, // 0t10201
			WordleClue(101) => 5, // 0t10202
			WordleClue(102) => 4, // 0t10210
			WordleClue(103) => 5, // 0t10211
			WordleClue(104) => 6, // 0t10212
			WordleClue(105) => 5, // 0t10220
			WordleClue(106) => 6, // 0t10221
			WordleClue(107) => 7, // 0t10222
			WordleClue(108) => 2, // 0t11000
			WordleClue(109) => 3, // 0t11001
			WordleClue(110) => 4, // 0t11002
			WordleClue(111) => 3, // 0t11010
			WordleClue(112) => 4, // 0t11011
			WordleClue(113) => 5, // 0t11012
			WordleClue(114) => 4, // 0t11020
			WordleClue(115) => 5, // 0t11021
			WordleClue(116) => 6, // 0t11022
			WordleClue(117) => 3, // 0t11100
			WordleClue(118) => 4, // 0t11101
			WordleClue(119) => 5, // 0t11102
			WordleClue(120) => 4, // 0t11110
			WordleClue(121) => 5, // 0t11111
			WordleClue(122) => 6, // 0t11112
			WordleClue(123) => 5, // 0t11120
			WordleClue(124) => 6, // 0t11121
			WordleClue(125) => 7, // 0t11122
			WordleClue(126) => 4, // 0t11200
			WordleClue(127) => 5, // 0t11201
			WordleClue(128) => 6, // 0t11202
			WordleClue(129) => 5, // 0t11210
			WordleClue(130) => 6, // 0t11211
			WordleClue(131) => 7, // 0t11212
			WordleClue(132) => 6, // 0t11220
			WordleClue(133) => 7, // 0t11221
			WordleClue(134) => 8, // 0t11222
			WordleClue(135) => 3, // 0t12000
			WordleClue(136) => 4, // 0t12001
			WordleClue(137) => 5, // 0t12002
			WordleClue(138) => 4, // 0t12010
			WordleClue(139) => 5, // 0t12011
			WordleClue(140) => 6, // 0t12012
			WordleClue(141) => 5, // 0t12020
			WordleClue(142) => 6, // 0t12021
			WordleClue(143) => 7, // 0t12022
			WordleClue(144) => 4, // 0t12100
			WordleClue(145) => 5, // 0t12101
			WordleClue(146) => 6, // 0t12102
			WordleClue(147) => 5, // 0t12110
			WordleClue(148) => 6, // 0t12111
			WordleClue(149) => 7, // 0t12112
			WordleClue(150) => 6, // 0t12120
			WordleClue(151) => 7, // 0t12121
			WordleClue(152) => 8, // 0t12122
			WordleClue(153) => 5, // 0t12200
			WordleClue(154) => 6, // 0t12201
			WordleClue(155) => 7, // 0t12202
			WordleClue(156) => 6, // 0t12210
			WordleClue(157) => 7, // 0t12211
			WordleClue(158) => 8, // 0t12212
			WordleClue(159) => 7, // 0t12220
			WordleClue(160) => 8, // 0t12221
			WordleClue(161) => 9, // 0t12222
			WordleClue(162) => 2, // 0t20000
			WordleClue(163) => 3, // 0t20001
			WordleClue(164) => 4, // 0t20002
			WordleClue(165) => 3, // 0t20010
			WordleClue(166) => 4, // 0t20011
			WordleClue(167) => 5, // 0t20012
			WordleClue(168) => 4, // 0t20020
			WordleClue(169) => 5, // 0t20021
			WordleClue(170) => 6, // 0t20022
			WordleClue(171) => 3, // 0t20100
			WordleClue(172) => 4, // 0t20101
			WordleClue(173) => 5, // 0t20102
			WordleClue(174) => 4, // 0t20110
			WordleClue(175) => 5, // 0t20111
			WordleClue(176) => 6, // 0t20112
			WordleClue(177) => 5, // 0t20120
			WordleClue(178) => 6, // 0t20121
			WordleClue(179) => 7, // 0t20122
			WordleClue(180) => 4, // 0t20200
			WordleClue(181) => 5, // 0t20201
			WordleClue(182) => 6, // 0t20202
			WordleClue(183) => 5, // 0t20210
			WordleClue(184) => 6, // 0t20211
			WordleClue(185) => 7, // 0t20212
			WordleClue(186) => 6, // 0t20220
			WordleClue(187) => 7, // 0t20221
			WordleClue(188) => 8, // 0t20222
			WordleClue(189) => 3, // 0t21000
			WordleClue(190) => 4, // 0t21001
			WordleClue(191) => 5, // 0t21002
			WordleClue(192) => 4, // 0t21010
			WordleClue(193) => 5, // 0t21011
			WordleClue(194) => 6, // 0t21012
			WordleClue(195) => 5, // 0t21020
			WordleClue(196) => 6, // 0t21021
			WordleClue(197) => 7, // 0t21022
			WordleClue(198) => 4, // 0t21100
			WordleClue(199) => 5, // 0t21101
			WordleClue(200) => 6, // 0t21102
			WordleClue(201) => 5, // 0t21110
			WordleClue(202) => 6, // 0t21111
			WordleClue(203) => 7, // 0t21112
			WordleClue(204) => 6, // 0t21120
			WordleClue(205) => 7, // 0t21121
			WordleClue(206) => 8, // 0t21122
			WordleClue(207) => 5, // 0t21200
			WordleClue(208) => 6, // 0t21201
			WordleClue(209) => 7, // 0t21202
			WordleClue(210) => 6, // 0t21210
			WordleClue(211) => 7, // 0t21211
			WordleClue(212) => 8, // 0t21212
			WordleClue(213) => 7, // 0t21220
			WordleClue(214) => 8, // 0t21221
			WordleClue(215) => 9, // 0t21222
			WordleClue(216) => 4, // 0t22000
			WordleClue(217) => 5, // 0t22001
			WordleClue(218) => 6, // 0t22002
			WordleClue(219) => 5, // 0t22010
			WordleClue(220) => 6, // 0t22011
			WordleClue(221) => 7, // 0t22012
			WordleClue(222) => 6, // 0t22020
			WordleClue(223) => 7, // 0t22021
			WordleClue(224) => 8, // 0t22022
			WordleClue(225) => 5, // 0t22100
			WordleClue(226) => 6, // 0t22101
			WordleClue(227) => 7, // 0t22102
			WordleClue(228) => 6, // 0t22110
			WordleClue(229) => 7, // 0t22111
			WordleClue(230) => 8, // 0t22112
			WordleClue(231) => 7, // 0t22120
			WordleClue(232) => 8, // 0t22121
			WordleClue(233) => 9, // 0t22122
			WordleClue(234) => 6, // 0t22200
			WordleClue(235) => 7, // 0t22201
			WordleClue(236) => 8, // 0t22202
			WordleClue(237) => 7, // 0t22210
			WordleClue(238) => 8, // 0t22211
			WordleClue(239) => 9, // 0t22212
			WordleClue(240) => 8, // 0t22220
			WordleClue(241) => 9, // 0t22221
			WordleClue(242) => 10, // 0t22222
			WordleClue(_) => unreachable!() // this can not happen bc it wont ever be >= 243
		}
	}
}
