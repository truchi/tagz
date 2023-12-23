// 🤖 This file is generated!

use crate::*;
/// The `<sub>` element's children.
#[derive(Clone)]
pub enum SubScriptChild {
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
impl From<Abbreviation> for SubScriptChild {
    fn from(child: Abbreviation) -> Self {
        SubScriptChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for SubScriptChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        SubScriptChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for SubScriptChild {
    fn from(child: Anchor) -> Self {
        SubScriptChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for SubScriptChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        SubScriptChild::Anchor(builder.build())
    }
}
impl From<Audio> for SubScriptChild {
    fn from(child: Audio) -> Self {
        SubScriptChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for SubScriptChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        SubScriptChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for SubScriptChild {
    fn from(child: BidirectionalIsolate) -> Self {
        SubScriptChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for SubScriptChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        SubScriptChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for SubScriptChild {
    fn from(child: BidirectionalOverride) -> Self {
        SubScriptChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for SubScriptChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        SubScriptChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for SubScriptChild {
    fn from(child: Bold) -> Self {
        SubScriptChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for SubScriptChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        SubScriptChild::Bold(builder.build())
    }
}
impl From<Button> for SubScriptChild {
    fn from(child: Button) -> Self {
        SubScriptChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for SubScriptChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        SubScriptChild::Button(builder.build())
    }
}
impl From<Canvas> for SubScriptChild {
    fn from(child: Canvas) -> Self {
        SubScriptChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for SubScriptChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        SubScriptChild::Canvas(builder.build())
    }
}
impl From<Cite> for SubScriptChild {
    fn from(child: Cite) -> Self {
        SubScriptChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for SubScriptChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        SubScriptChild::Cite(builder.build())
    }
}
impl From<Code> for SubScriptChild {
    fn from(child: Code) -> Self {
        SubScriptChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for SubScriptChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        SubScriptChild::Code(builder.build())
    }
}
impl From<Custom> for SubScriptChild {
    fn from(child: Custom) -> Self {
        SubScriptChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for SubScriptChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        SubScriptChild::Custom(builder.build())
    }
}
impl From<Data> for SubScriptChild {
    fn from(child: Data) -> Self {
        SubScriptChild::Data(child)
    }
}
impl From<builders::DataBuilder> for SubScriptChild {
    fn from(builder: builders::DataBuilder) -> Self {
        SubScriptChild::Data(builder.build())
    }
}
impl From<DataList> for SubScriptChild {
    fn from(child: DataList) -> Self {
        SubScriptChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for SubScriptChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        SubScriptChild::DataList(builder.build())
    }
}
impl From<Definition> for SubScriptChild {
    fn from(child: Definition) -> Self {
        SubScriptChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for SubScriptChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        SubScriptChild::Definition(builder.build())
    }
}
impl From<Deleted> for SubScriptChild {
    fn from(child: Deleted) -> Self {
        SubScriptChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for SubScriptChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        SubScriptChild::Deleted(builder.build())
    }
}
impl From<Embed> for SubScriptChild {
    fn from(child: Embed) -> Self {
        SubScriptChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for SubScriptChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        SubScriptChild::Embed(builder.build())
    }
}
impl From<Emphasis> for SubScriptChild {
    fn from(child: Emphasis) -> Self {
        SubScriptChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for SubScriptChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        SubScriptChild::Emphasis(builder.build())
    }
}
impl From<Image> for SubScriptChild {
    fn from(child: Image) -> Self {
        SubScriptChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for SubScriptChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        SubScriptChild::Image(builder.build())
    }
}
impl From<InlineFrame> for SubScriptChild {
    fn from(child: InlineFrame) -> Self {
        SubScriptChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for SubScriptChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        SubScriptChild::InlineFrame(builder.build())
    }
}
impl From<Input> for SubScriptChild {
    fn from(child: Input) -> Self {
        SubScriptChild::Input(child)
    }
}
impl From<builders::InputBuilder> for SubScriptChild {
    fn from(builder: builders::InputBuilder) -> Self {
        SubScriptChild::Input(builder.build())
    }
}
impl From<Inserted> for SubScriptChild {
    fn from(child: Inserted) -> Self {
        SubScriptChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for SubScriptChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        SubScriptChild::Inserted(builder.build())
    }
}
impl From<Italic> for SubScriptChild {
    fn from(child: Italic) -> Self {
        SubScriptChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for SubScriptChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        SubScriptChild::Italic(builder.build())
    }
}
impl From<Keyboard> for SubScriptChild {
    fn from(child: Keyboard) -> Self {
        SubScriptChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for SubScriptChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        SubScriptChild::Keyboard(builder.build())
    }
}
impl From<Label> for SubScriptChild {
    fn from(child: Label) -> Self {
        SubScriptChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for SubScriptChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        SubScriptChild::Label(builder.build())
    }
}
impl From<LineBreak> for SubScriptChild {
    fn from(child: LineBreak) -> Self {
        SubScriptChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for SubScriptChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        SubScriptChild::LineBreak(builder.build())
    }
}
impl From<Link> for SubScriptChild {
    fn from(child: Link) -> Self {
        SubScriptChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for SubScriptChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        SubScriptChild::Link(builder.build())
    }
}
impl From<Map> for SubScriptChild {
    fn from(child: Map) -> Self {
        SubScriptChild::Map(child)
    }
}
impl From<builders::MapBuilder> for SubScriptChild {
    fn from(builder: builders::MapBuilder) -> Self {
        SubScriptChild::Map(builder.build())
    }
}
impl From<MapArea> for SubScriptChild {
    fn from(child: MapArea) -> Self {
        SubScriptChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for SubScriptChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        SubScriptChild::MapArea(builder.build())
    }
}
impl From<Mark> for SubScriptChild {
    fn from(child: Mark) -> Self {
        SubScriptChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for SubScriptChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        SubScriptChild::Mark(builder.build())
    }
}
impl From<Metadata> for SubScriptChild {
    fn from(child: Metadata) -> Self {
        SubScriptChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for SubScriptChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        SubScriptChild::Metadata(builder.build())
    }
}
impl From<Meter> for SubScriptChild {
    fn from(child: Meter) -> Self {
        SubScriptChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for SubScriptChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        SubScriptChild::Meter(builder.build())
    }
}
impl From<NoScript> for SubScriptChild {
    fn from(child: NoScript) -> Self {
        SubScriptChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for SubScriptChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        SubScriptChild::NoScript(builder.build())
    }
}
impl From<Object> for SubScriptChild {
    fn from(child: Object) -> Self {
        SubScriptChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for SubScriptChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        SubScriptChild::Object(builder.build())
    }
}
impl From<Output> for SubScriptChild {
    fn from(child: Output) -> Self {
        SubScriptChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for SubScriptChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        SubScriptChild::Output(builder.build())
    }
}
impl From<Picture> for SubScriptChild {
    fn from(child: Picture) -> Self {
        SubScriptChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for SubScriptChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        SubScriptChild::Picture(builder.build())
    }
}
impl From<Progress> for SubScriptChild {
    fn from(child: Progress) -> Self {
        SubScriptChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for SubScriptChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        SubScriptChild::Progress(builder.build())
    }
}
impl From<Quote> for SubScriptChild {
    fn from(child: Quote) -> Self {
        SubScriptChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for SubScriptChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        SubScriptChild::Quote(builder.build())
    }
}
impl From<Ruby> for SubScriptChild {
    fn from(child: Ruby) -> Self {
        SubScriptChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for SubScriptChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        SubScriptChild::Ruby(builder.build())
    }
}
impl From<Sample> for SubScriptChild {
    fn from(child: Sample) -> Self {
        SubScriptChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for SubScriptChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        SubScriptChild::Sample(builder.build())
    }
}
impl From<Script> for SubScriptChild {
    fn from(child: Script) -> Self {
        SubScriptChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for SubScriptChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        SubScriptChild::Script(builder.build())
    }
}
impl From<Select> for SubScriptChild {
    fn from(child: Select) -> Self {
        SubScriptChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for SubScriptChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        SubScriptChild::Select(builder.build())
    }
}
impl From<Slot> for SubScriptChild {
    fn from(child: Slot) -> Self {
        SubScriptChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for SubScriptChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        SubScriptChild::Slot(builder.build())
    }
}
impl From<Small> for SubScriptChild {
    fn from(child: Small) -> Self {
        SubScriptChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for SubScriptChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        SubScriptChild::Small(builder.build())
    }
}
impl From<Span> for SubScriptChild {
    fn from(child: Span) -> Self {
        SubScriptChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for SubScriptChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        SubScriptChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for SubScriptChild {
    fn from(child: StrikeThrough) -> Self {
        SubScriptChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for SubScriptChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        SubScriptChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for SubScriptChild {
    fn from(child: Strong) -> Self {
        SubScriptChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for SubScriptChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        SubScriptChild::Strong(builder.build())
    }
}
impl From<SubScript> for SubScriptChild {
    fn from(child: SubScript) -> Self {
        SubScriptChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for SubScriptChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        SubScriptChild::SubScript(builder.build())
    }
}
impl From<SupScript> for SubScriptChild {
    fn from(child: SupScript) -> Self {
        SubScriptChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for SubScriptChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        SubScriptChild::SupScript(builder.build())
    }
}
impl From<Template> for SubScriptChild {
    fn from(child: Template) -> Self {
        SubScriptChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for SubScriptChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        SubScriptChild::Template(builder.build())
    }
}
impl From<TextArea> for SubScriptChild {
    fn from(child: TextArea) -> Self {
        SubScriptChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for SubScriptChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        SubScriptChild::TextArea(builder.build())
    }
}
impl From<Time> for SubScriptChild {
    fn from(child: Time) -> Self {
        SubScriptChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for SubScriptChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        SubScriptChild::Time(builder.build())
    }
}
impl From<Underline> for SubScriptChild {
    fn from(child: Underline) -> Self {
        SubScriptChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for SubScriptChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        SubScriptChild::Underline(builder.build())
    }
}
impl From<Variable> for SubScriptChild {
    fn from(child: Variable) -> Self {
        SubScriptChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for SubScriptChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        SubScriptChild::Variable(builder.build())
    }
}
impl From<Video> for SubScriptChild {
    fn from(child: Video) -> Self {
        SubScriptChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for SubScriptChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        SubScriptChild::Video(builder.build())
    }
}
impl From<WordBreak> for SubScriptChild {
    fn from(child: WordBreak) -> Self {
        SubScriptChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for SubScriptChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        SubScriptChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for SubScriptChild {
    fn from(s: &'static str) -> Self {
        SubScriptChild::Text(s.into())
    }
}
impl From<String> for SubScriptChild {
    fn from(s: String) -> Self {
        SubScriptChild::Text(s.into())
    }
}
impl From<CowStr> for SubScriptChild {
    fn from(s: CowStr) -> Self {
        SubScriptChild::Text(s)
    }
}
impl std::fmt::Debug for SubScriptChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubScriptChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            SubScriptChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Button(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Code(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Data(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Image(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Input(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Label(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Link(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Map(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Object(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Output(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Script(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Select(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Small(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Span(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Template(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Time(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Video(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            SubScriptChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for SubScriptChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubScriptChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Audio(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            SubScriptChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            SubScriptChild::Bold(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Button(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Cite(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Code(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Custom(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Data(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::DataList(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Definition(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Embed(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Image(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Input(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Italic(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Label(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Link(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Map(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Mark(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Meter(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Object(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Output(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Picture(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Progress(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Quote(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Sample(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Script(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Select(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Slot(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Small(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Span(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Strong(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Template(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Time(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Underline(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Variable(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Video(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            SubScriptChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
