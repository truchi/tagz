// 🤖 This file is generated!

use crate::*;
/// The `<sup>` element's children.
#[derive(Clone)]
pub enum SupScriptChild {
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
impl From<Abbreviation> for SupScriptChild {
    fn from(child: Abbreviation) -> Self {
        SupScriptChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for SupScriptChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        SupScriptChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for SupScriptChild {
    fn from(child: Anchor) -> Self {
        SupScriptChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for SupScriptChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        SupScriptChild::Anchor(builder.build())
    }
}
impl From<Audio> for SupScriptChild {
    fn from(child: Audio) -> Self {
        SupScriptChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for SupScriptChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        SupScriptChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for SupScriptChild {
    fn from(child: BidirectionalIsolate) -> Self {
        SupScriptChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for SupScriptChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        SupScriptChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for SupScriptChild {
    fn from(child: BidirectionalOverride) -> Self {
        SupScriptChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for SupScriptChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        SupScriptChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for SupScriptChild {
    fn from(child: Bold) -> Self {
        SupScriptChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for SupScriptChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        SupScriptChild::Bold(builder.build())
    }
}
impl From<Button> for SupScriptChild {
    fn from(child: Button) -> Self {
        SupScriptChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for SupScriptChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        SupScriptChild::Button(builder.build())
    }
}
impl From<Canvas> for SupScriptChild {
    fn from(child: Canvas) -> Self {
        SupScriptChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for SupScriptChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        SupScriptChild::Canvas(builder.build())
    }
}
impl From<Cite> for SupScriptChild {
    fn from(child: Cite) -> Self {
        SupScriptChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for SupScriptChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        SupScriptChild::Cite(builder.build())
    }
}
impl From<Code> for SupScriptChild {
    fn from(child: Code) -> Self {
        SupScriptChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for SupScriptChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        SupScriptChild::Code(builder.build())
    }
}
impl From<Custom> for SupScriptChild {
    fn from(child: Custom) -> Self {
        SupScriptChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for SupScriptChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        SupScriptChild::Custom(builder.build())
    }
}
impl From<Data> for SupScriptChild {
    fn from(child: Data) -> Self {
        SupScriptChild::Data(child)
    }
}
impl From<builders::DataBuilder> for SupScriptChild {
    fn from(builder: builders::DataBuilder) -> Self {
        SupScriptChild::Data(builder.build())
    }
}
impl From<DataList> for SupScriptChild {
    fn from(child: DataList) -> Self {
        SupScriptChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for SupScriptChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        SupScriptChild::DataList(builder.build())
    }
}
impl From<Definition> for SupScriptChild {
    fn from(child: Definition) -> Self {
        SupScriptChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for SupScriptChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        SupScriptChild::Definition(builder.build())
    }
}
impl From<Deleted> for SupScriptChild {
    fn from(child: Deleted) -> Self {
        SupScriptChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for SupScriptChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        SupScriptChild::Deleted(builder.build())
    }
}
impl From<Embed> for SupScriptChild {
    fn from(child: Embed) -> Self {
        SupScriptChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for SupScriptChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        SupScriptChild::Embed(builder.build())
    }
}
impl From<Emphasis> for SupScriptChild {
    fn from(child: Emphasis) -> Self {
        SupScriptChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for SupScriptChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        SupScriptChild::Emphasis(builder.build())
    }
}
impl From<Image> for SupScriptChild {
    fn from(child: Image) -> Self {
        SupScriptChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for SupScriptChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        SupScriptChild::Image(builder.build())
    }
}
impl From<InlineFrame> for SupScriptChild {
    fn from(child: InlineFrame) -> Self {
        SupScriptChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for SupScriptChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        SupScriptChild::InlineFrame(builder.build())
    }
}
impl From<Input> for SupScriptChild {
    fn from(child: Input) -> Self {
        SupScriptChild::Input(child)
    }
}
impl From<builders::InputBuilder> for SupScriptChild {
    fn from(builder: builders::InputBuilder) -> Self {
        SupScriptChild::Input(builder.build())
    }
}
impl From<Inserted> for SupScriptChild {
    fn from(child: Inserted) -> Self {
        SupScriptChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for SupScriptChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        SupScriptChild::Inserted(builder.build())
    }
}
impl From<Italic> for SupScriptChild {
    fn from(child: Italic) -> Self {
        SupScriptChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for SupScriptChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        SupScriptChild::Italic(builder.build())
    }
}
impl From<Keyboard> for SupScriptChild {
    fn from(child: Keyboard) -> Self {
        SupScriptChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for SupScriptChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        SupScriptChild::Keyboard(builder.build())
    }
}
impl From<Label> for SupScriptChild {
    fn from(child: Label) -> Self {
        SupScriptChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for SupScriptChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        SupScriptChild::Label(builder.build())
    }
}
impl From<LineBreak> for SupScriptChild {
    fn from(child: LineBreak) -> Self {
        SupScriptChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for SupScriptChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        SupScriptChild::LineBreak(builder.build())
    }
}
impl From<Link> for SupScriptChild {
    fn from(child: Link) -> Self {
        SupScriptChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for SupScriptChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        SupScriptChild::Link(builder.build())
    }
}
impl From<Map> for SupScriptChild {
    fn from(child: Map) -> Self {
        SupScriptChild::Map(child)
    }
}
impl From<builders::MapBuilder> for SupScriptChild {
    fn from(builder: builders::MapBuilder) -> Self {
        SupScriptChild::Map(builder.build())
    }
}
impl From<MapArea> for SupScriptChild {
    fn from(child: MapArea) -> Self {
        SupScriptChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for SupScriptChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        SupScriptChild::MapArea(builder.build())
    }
}
impl From<Mark> for SupScriptChild {
    fn from(child: Mark) -> Self {
        SupScriptChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for SupScriptChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        SupScriptChild::Mark(builder.build())
    }
}
impl From<Metadata> for SupScriptChild {
    fn from(child: Metadata) -> Self {
        SupScriptChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for SupScriptChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        SupScriptChild::Metadata(builder.build())
    }
}
impl From<Meter> for SupScriptChild {
    fn from(child: Meter) -> Self {
        SupScriptChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for SupScriptChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        SupScriptChild::Meter(builder.build())
    }
}
impl From<NoScript> for SupScriptChild {
    fn from(child: NoScript) -> Self {
        SupScriptChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for SupScriptChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        SupScriptChild::NoScript(builder.build())
    }
}
impl From<Object> for SupScriptChild {
    fn from(child: Object) -> Self {
        SupScriptChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for SupScriptChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        SupScriptChild::Object(builder.build())
    }
}
impl From<Output> for SupScriptChild {
    fn from(child: Output) -> Self {
        SupScriptChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for SupScriptChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        SupScriptChild::Output(builder.build())
    }
}
impl From<Picture> for SupScriptChild {
    fn from(child: Picture) -> Self {
        SupScriptChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for SupScriptChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        SupScriptChild::Picture(builder.build())
    }
}
impl From<Progress> for SupScriptChild {
    fn from(child: Progress) -> Self {
        SupScriptChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for SupScriptChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        SupScriptChild::Progress(builder.build())
    }
}
impl From<Quote> for SupScriptChild {
    fn from(child: Quote) -> Self {
        SupScriptChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for SupScriptChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        SupScriptChild::Quote(builder.build())
    }
}
impl From<Ruby> for SupScriptChild {
    fn from(child: Ruby) -> Self {
        SupScriptChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for SupScriptChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        SupScriptChild::Ruby(builder.build())
    }
}
impl From<Sample> for SupScriptChild {
    fn from(child: Sample) -> Self {
        SupScriptChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for SupScriptChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        SupScriptChild::Sample(builder.build())
    }
}
impl From<Script> for SupScriptChild {
    fn from(child: Script) -> Self {
        SupScriptChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for SupScriptChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        SupScriptChild::Script(builder.build())
    }
}
impl From<Select> for SupScriptChild {
    fn from(child: Select) -> Self {
        SupScriptChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for SupScriptChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        SupScriptChild::Select(builder.build())
    }
}
impl From<Slot> for SupScriptChild {
    fn from(child: Slot) -> Self {
        SupScriptChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for SupScriptChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        SupScriptChild::Slot(builder.build())
    }
}
impl From<Small> for SupScriptChild {
    fn from(child: Small) -> Self {
        SupScriptChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for SupScriptChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        SupScriptChild::Small(builder.build())
    }
}
impl From<Span> for SupScriptChild {
    fn from(child: Span) -> Self {
        SupScriptChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for SupScriptChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        SupScriptChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for SupScriptChild {
    fn from(child: StrikeThrough) -> Self {
        SupScriptChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for SupScriptChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        SupScriptChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for SupScriptChild {
    fn from(child: Strong) -> Self {
        SupScriptChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for SupScriptChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        SupScriptChild::Strong(builder.build())
    }
}
impl From<SubScript> for SupScriptChild {
    fn from(child: SubScript) -> Self {
        SupScriptChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for SupScriptChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        SupScriptChild::SubScript(builder.build())
    }
}
impl From<SupScript> for SupScriptChild {
    fn from(child: SupScript) -> Self {
        SupScriptChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for SupScriptChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        SupScriptChild::SupScript(builder.build())
    }
}
impl From<Template> for SupScriptChild {
    fn from(child: Template) -> Self {
        SupScriptChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for SupScriptChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        SupScriptChild::Template(builder.build())
    }
}
impl From<TextArea> for SupScriptChild {
    fn from(child: TextArea) -> Self {
        SupScriptChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for SupScriptChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        SupScriptChild::TextArea(builder.build())
    }
}
impl From<Time> for SupScriptChild {
    fn from(child: Time) -> Self {
        SupScriptChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for SupScriptChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        SupScriptChild::Time(builder.build())
    }
}
impl From<Underline> for SupScriptChild {
    fn from(child: Underline) -> Self {
        SupScriptChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for SupScriptChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        SupScriptChild::Underline(builder.build())
    }
}
impl From<Variable> for SupScriptChild {
    fn from(child: Variable) -> Self {
        SupScriptChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for SupScriptChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        SupScriptChild::Variable(builder.build())
    }
}
impl From<Video> for SupScriptChild {
    fn from(child: Video) -> Self {
        SupScriptChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for SupScriptChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        SupScriptChild::Video(builder.build())
    }
}
impl From<WordBreak> for SupScriptChild {
    fn from(child: WordBreak) -> Self {
        SupScriptChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for SupScriptChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        SupScriptChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for SupScriptChild {
    fn from(s: &'static str) -> Self {
        SupScriptChild::Text(s.into())
    }
}
impl From<String> for SupScriptChild {
    fn from(s: String) -> Self {
        SupScriptChild::Text(s.into())
    }
}
impl From<CowStr> for SupScriptChild {
    fn from(s: CowStr) -> Self {
        SupScriptChild::Text(s)
    }
}
impl std::fmt::Debug for SupScriptChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupScriptChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            SupScriptChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Button(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Code(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Data(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Image(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Input(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Label(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Link(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Map(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Object(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Output(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Script(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Select(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Small(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Span(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Template(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Time(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Video(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            SupScriptChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for SupScriptChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupScriptChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Audio(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            SupScriptChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            SupScriptChild::Bold(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Button(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Cite(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Code(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Custom(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Data(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::DataList(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Definition(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Embed(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Image(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Input(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Italic(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Label(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Link(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Map(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Mark(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Meter(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Object(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Output(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Picture(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Progress(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Quote(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Sample(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Script(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Select(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Slot(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Small(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Span(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Strong(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Template(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Time(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Underline(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Variable(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Video(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            SupScriptChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
