// 🤖 This file is generated!

use crate::*;
/// The `<rt>` element's children.
#[derive(Clone)]
pub enum RubyTextChild {
    Abbreviation(Abbreviation),
    Anchor(Anchor),
    Audio(Audio),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    Bold(Bold),
    Button(Button),
    Canvas(Canvas),
    Cite(Cite),
    Code(Code),
    Custom(Custom),
    Data(Data),
    DataList(DataList),
    Definition(Definition),
    Deleted(Deleted),
    Embed(Embed),
    Emphasis(Emphasis),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    Map(Map),
    MapArea(MapArea),
    Mark(Mark),
    Metadata(Metadata),
    Meter(Meter),
    NoScript(NoScript),
    Object(Object),
    Output(Output),
    Picture(Picture),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    Sample(Sample),
    Script(Script),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    SubScript(SubScript),
    SupScript(SupScript),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Underline(Underline),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
    Text(CowStr),
}
impl From<Abbreviation> for RubyTextChild {
    fn from(child: Abbreviation) -> Self {
        RubyTextChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for RubyTextChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        RubyTextChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for RubyTextChild {
    fn from(child: Anchor) -> Self {
        RubyTextChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for RubyTextChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        RubyTextChild::Anchor(builder.build())
    }
}
impl From<Audio> for RubyTextChild {
    fn from(child: Audio) -> Self {
        RubyTextChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for RubyTextChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        RubyTextChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for RubyTextChild {
    fn from(child: BidirectionalIsolate) -> Self {
        RubyTextChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for RubyTextChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        RubyTextChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for RubyTextChild {
    fn from(child: BidirectionalOverride) -> Self {
        RubyTextChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for RubyTextChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        RubyTextChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for RubyTextChild {
    fn from(child: Bold) -> Self {
        RubyTextChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for RubyTextChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        RubyTextChild::Bold(builder.build())
    }
}
impl From<Button> for RubyTextChild {
    fn from(child: Button) -> Self {
        RubyTextChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for RubyTextChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        RubyTextChild::Button(builder.build())
    }
}
impl From<Canvas> for RubyTextChild {
    fn from(child: Canvas) -> Self {
        RubyTextChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for RubyTextChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        RubyTextChild::Canvas(builder.build())
    }
}
impl From<Cite> for RubyTextChild {
    fn from(child: Cite) -> Self {
        RubyTextChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for RubyTextChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        RubyTextChild::Cite(builder.build())
    }
}
impl From<Code> for RubyTextChild {
    fn from(child: Code) -> Self {
        RubyTextChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for RubyTextChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        RubyTextChild::Code(builder.build())
    }
}
impl From<Custom> for RubyTextChild {
    fn from(child: Custom) -> Self {
        RubyTextChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for RubyTextChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        RubyTextChild::Custom(builder.build())
    }
}
impl From<Data> for RubyTextChild {
    fn from(child: Data) -> Self {
        RubyTextChild::Data(child)
    }
}
impl From<builders::DataBuilder> for RubyTextChild {
    fn from(builder: builders::DataBuilder) -> Self {
        RubyTextChild::Data(builder.build())
    }
}
impl From<DataList> for RubyTextChild {
    fn from(child: DataList) -> Self {
        RubyTextChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for RubyTextChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        RubyTextChild::DataList(builder.build())
    }
}
impl From<Definition> for RubyTextChild {
    fn from(child: Definition) -> Self {
        RubyTextChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for RubyTextChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        RubyTextChild::Definition(builder.build())
    }
}
impl From<Deleted> for RubyTextChild {
    fn from(child: Deleted) -> Self {
        RubyTextChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for RubyTextChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        RubyTextChild::Deleted(builder.build())
    }
}
impl From<Embed> for RubyTextChild {
    fn from(child: Embed) -> Self {
        RubyTextChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for RubyTextChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        RubyTextChild::Embed(builder.build())
    }
}
impl From<Emphasis> for RubyTextChild {
    fn from(child: Emphasis) -> Self {
        RubyTextChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for RubyTextChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        RubyTextChild::Emphasis(builder.build())
    }
}
impl From<Image> for RubyTextChild {
    fn from(child: Image) -> Self {
        RubyTextChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for RubyTextChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        RubyTextChild::Image(builder.build())
    }
}
impl From<InlineFrame> for RubyTextChild {
    fn from(child: InlineFrame) -> Self {
        RubyTextChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for RubyTextChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        RubyTextChild::InlineFrame(builder.build())
    }
}
impl From<Input> for RubyTextChild {
    fn from(child: Input) -> Self {
        RubyTextChild::Input(child)
    }
}
impl From<builders::InputBuilder> for RubyTextChild {
    fn from(builder: builders::InputBuilder) -> Self {
        RubyTextChild::Input(builder.build())
    }
}
impl From<Inserted> for RubyTextChild {
    fn from(child: Inserted) -> Self {
        RubyTextChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for RubyTextChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        RubyTextChild::Inserted(builder.build())
    }
}
impl From<Italic> for RubyTextChild {
    fn from(child: Italic) -> Self {
        RubyTextChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for RubyTextChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        RubyTextChild::Italic(builder.build())
    }
}
impl From<Keyboard> for RubyTextChild {
    fn from(child: Keyboard) -> Self {
        RubyTextChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for RubyTextChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        RubyTextChild::Keyboard(builder.build())
    }
}
impl From<Label> for RubyTextChild {
    fn from(child: Label) -> Self {
        RubyTextChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for RubyTextChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        RubyTextChild::Label(builder.build())
    }
}
impl From<LineBreak> for RubyTextChild {
    fn from(child: LineBreak) -> Self {
        RubyTextChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for RubyTextChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        RubyTextChild::LineBreak(builder.build())
    }
}
impl From<Link> for RubyTextChild {
    fn from(child: Link) -> Self {
        RubyTextChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for RubyTextChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        RubyTextChild::Link(builder.build())
    }
}
impl From<Map> for RubyTextChild {
    fn from(child: Map) -> Self {
        RubyTextChild::Map(child)
    }
}
impl From<builders::MapBuilder> for RubyTextChild {
    fn from(builder: builders::MapBuilder) -> Self {
        RubyTextChild::Map(builder.build())
    }
}
impl From<MapArea> for RubyTextChild {
    fn from(child: MapArea) -> Self {
        RubyTextChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for RubyTextChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        RubyTextChild::MapArea(builder.build())
    }
}
impl From<Mark> for RubyTextChild {
    fn from(child: Mark) -> Self {
        RubyTextChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for RubyTextChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        RubyTextChild::Mark(builder.build())
    }
}
impl From<Metadata> for RubyTextChild {
    fn from(child: Metadata) -> Self {
        RubyTextChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for RubyTextChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        RubyTextChild::Metadata(builder.build())
    }
}
impl From<Meter> for RubyTextChild {
    fn from(child: Meter) -> Self {
        RubyTextChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for RubyTextChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        RubyTextChild::Meter(builder.build())
    }
}
impl From<NoScript> for RubyTextChild {
    fn from(child: NoScript) -> Self {
        RubyTextChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for RubyTextChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        RubyTextChild::NoScript(builder.build())
    }
}
impl From<Object> for RubyTextChild {
    fn from(child: Object) -> Self {
        RubyTextChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for RubyTextChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        RubyTextChild::Object(builder.build())
    }
}
impl From<Output> for RubyTextChild {
    fn from(child: Output) -> Self {
        RubyTextChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for RubyTextChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        RubyTextChild::Output(builder.build())
    }
}
impl From<Picture> for RubyTextChild {
    fn from(child: Picture) -> Self {
        RubyTextChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for RubyTextChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        RubyTextChild::Picture(builder.build())
    }
}
impl From<Progress> for RubyTextChild {
    fn from(child: Progress) -> Self {
        RubyTextChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for RubyTextChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        RubyTextChild::Progress(builder.build())
    }
}
impl From<Quote> for RubyTextChild {
    fn from(child: Quote) -> Self {
        RubyTextChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for RubyTextChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        RubyTextChild::Quote(builder.build())
    }
}
impl From<Ruby> for RubyTextChild {
    fn from(child: Ruby) -> Self {
        RubyTextChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for RubyTextChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        RubyTextChild::Ruby(builder.build())
    }
}
impl From<Sample> for RubyTextChild {
    fn from(child: Sample) -> Self {
        RubyTextChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for RubyTextChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        RubyTextChild::Sample(builder.build())
    }
}
impl From<Script> for RubyTextChild {
    fn from(child: Script) -> Self {
        RubyTextChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for RubyTextChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        RubyTextChild::Script(builder.build())
    }
}
impl From<Select> for RubyTextChild {
    fn from(child: Select) -> Self {
        RubyTextChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for RubyTextChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        RubyTextChild::Select(builder.build())
    }
}
impl From<Slot> for RubyTextChild {
    fn from(child: Slot) -> Self {
        RubyTextChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for RubyTextChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        RubyTextChild::Slot(builder.build())
    }
}
impl From<Small> for RubyTextChild {
    fn from(child: Small) -> Self {
        RubyTextChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for RubyTextChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        RubyTextChild::Small(builder.build())
    }
}
impl From<Span> for RubyTextChild {
    fn from(child: Span) -> Self {
        RubyTextChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for RubyTextChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        RubyTextChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for RubyTextChild {
    fn from(child: StrikeThrough) -> Self {
        RubyTextChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for RubyTextChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        RubyTextChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for RubyTextChild {
    fn from(child: Strong) -> Self {
        RubyTextChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for RubyTextChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        RubyTextChild::Strong(builder.build())
    }
}
impl From<SubScript> for RubyTextChild {
    fn from(child: SubScript) -> Self {
        RubyTextChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for RubyTextChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        RubyTextChild::SubScript(builder.build())
    }
}
impl From<SupScript> for RubyTextChild {
    fn from(child: SupScript) -> Self {
        RubyTextChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for RubyTextChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        RubyTextChild::SupScript(builder.build())
    }
}
impl From<Template> for RubyTextChild {
    fn from(child: Template) -> Self {
        RubyTextChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for RubyTextChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        RubyTextChild::Template(builder.build())
    }
}
impl From<TextArea> for RubyTextChild {
    fn from(child: TextArea) -> Self {
        RubyTextChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for RubyTextChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        RubyTextChild::TextArea(builder.build())
    }
}
impl From<Time> for RubyTextChild {
    fn from(child: Time) -> Self {
        RubyTextChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for RubyTextChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        RubyTextChild::Time(builder.build())
    }
}
impl From<Underline> for RubyTextChild {
    fn from(child: Underline) -> Self {
        RubyTextChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for RubyTextChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        RubyTextChild::Underline(builder.build())
    }
}
impl From<Variable> for RubyTextChild {
    fn from(child: Variable) -> Self {
        RubyTextChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for RubyTextChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        RubyTextChild::Variable(builder.build())
    }
}
impl From<Video> for RubyTextChild {
    fn from(child: Video) -> Self {
        RubyTextChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for RubyTextChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        RubyTextChild::Video(builder.build())
    }
}
impl From<WordBreak> for RubyTextChild {
    fn from(child: WordBreak) -> Self {
        RubyTextChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for RubyTextChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        RubyTextChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for RubyTextChild {
    fn from(s: &'static str) -> Self {
        RubyTextChild::Text(s.into())
    }
}
impl From<String> for RubyTextChild {
    fn from(s: String) -> Self {
        RubyTextChild::Text(s.into())
    }
}
impl From<CowStr> for RubyTextChild {
    fn from(s: CowStr) -> Self {
        RubyTextChild::Text(s)
    }
}
impl std::fmt::Debug for RubyTextChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RubyTextChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Button(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Code(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Data(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Image(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Input(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Label(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Link(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Map(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Object(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Output(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Script(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Select(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Small(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Span(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Template(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Time(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Video(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            RubyTextChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for RubyTextChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RubyTextChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Audio(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            RubyTextChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            RubyTextChild::Bold(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Button(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Cite(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Code(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Custom(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Data(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::DataList(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Definition(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Embed(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Image(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Input(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Italic(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Label(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Link(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Map(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Mark(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Meter(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Object(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Output(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Picture(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Progress(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Quote(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Sample(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Script(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Select(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Slot(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Small(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Span(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Strong(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Template(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Time(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Underline(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Variable(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Video(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            RubyTextChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
