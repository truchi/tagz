// 🤖 This file is generated!

use crate::*;
/// The `<h3>` element's children.
#[derive(Clone)]
pub enum Heading3Child {
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
impl From<Abbreviation> for Heading3Child {
    fn from(child: Abbreviation) -> Self {
        Heading3Child::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for Heading3Child {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        Heading3Child::Abbreviation(builder.build())
    }
}
impl From<Anchor> for Heading3Child {
    fn from(child: Anchor) -> Self {
        Heading3Child::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for Heading3Child {
    fn from(builder: builders::AnchorBuilder) -> Self {
        Heading3Child::Anchor(builder.build())
    }
}
impl From<Audio> for Heading3Child {
    fn from(child: Audio) -> Self {
        Heading3Child::Audio(child)
    }
}
impl From<builders::AudioBuilder> for Heading3Child {
    fn from(builder: builders::AudioBuilder) -> Self {
        Heading3Child::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for Heading3Child {
    fn from(child: BidirectionalIsolate) -> Self {
        Heading3Child::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for Heading3Child {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        Heading3Child::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for Heading3Child {
    fn from(child: BidirectionalOverride) -> Self {
        Heading3Child::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for Heading3Child {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        Heading3Child::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for Heading3Child {
    fn from(child: Bold) -> Self {
        Heading3Child::Bold(child)
    }
}
impl From<builders::BoldBuilder> for Heading3Child {
    fn from(builder: builders::BoldBuilder) -> Self {
        Heading3Child::Bold(builder.build())
    }
}
impl From<Button> for Heading3Child {
    fn from(child: Button) -> Self {
        Heading3Child::Button(child)
    }
}
impl From<builders::ButtonBuilder> for Heading3Child {
    fn from(builder: builders::ButtonBuilder) -> Self {
        Heading3Child::Button(builder.build())
    }
}
impl From<Canvas> for Heading3Child {
    fn from(child: Canvas) -> Self {
        Heading3Child::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for Heading3Child {
    fn from(builder: builders::CanvasBuilder) -> Self {
        Heading3Child::Canvas(builder.build())
    }
}
impl From<Cite> for Heading3Child {
    fn from(child: Cite) -> Self {
        Heading3Child::Cite(child)
    }
}
impl From<builders::CiteBuilder> for Heading3Child {
    fn from(builder: builders::CiteBuilder) -> Self {
        Heading3Child::Cite(builder.build())
    }
}
impl From<Code> for Heading3Child {
    fn from(child: Code) -> Self {
        Heading3Child::Code(child)
    }
}
impl From<builders::CodeBuilder> for Heading3Child {
    fn from(builder: builders::CodeBuilder) -> Self {
        Heading3Child::Code(builder.build())
    }
}
impl From<Custom> for Heading3Child {
    fn from(child: Custom) -> Self {
        Heading3Child::Custom(child)
    }
}
impl From<builders::CustomBuilder> for Heading3Child {
    fn from(builder: builders::CustomBuilder) -> Self {
        Heading3Child::Custom(builder.build())
    }
}
impl From<Data> for Heading3Child {
    fn from(child: Data) -> Self {
        Heading3Child::Data(child)
    }
}
impl From<builders::DataBuilder> for Heading3Child {
    fn from(builder: builders::DataBuilder) -> Self {
        Heading3Child::Data(builder.build())
    }
}
impl From<DataList> for Heading3Child {
    fn from(child: DataList) -> Self {
        Heading3Child::DataList(child)
    }
}
impl From<builders::DataListBuilder> for Heading3Child {
    fn from(builder: builders::DataListBuilder) -> Self {
        Heading3Child::DataList(builder.build())
    }
}
impl From<Definition> for Heading3Child {
    fn from(child: Definition) -> Self {
        Heading3Child::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for Heading3Child {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        Heading3Child::Definition(builder.build())
    }
}
impl From<Deleted> for Heading3Child {
    fn from(child: Deleted) -> Self {
        Heading3Child::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for Heading3Child {
    fn from(builder: builders::DeletedBuilder) -> Self {
        Heading3Child::Deleted(builder.build())
    }
}
impl From<Embed> for Heading3Child {
    fn from(child: Embed) -> Self {
        Heading3Child::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for Heading3Child {
    fn from(builder: builders::EmbedBuilder) -> Self {
        Heading3Child::Embed(builder.build())
    }
}
impl From<Emphasis> for Heading3Child {
    fn from(child: Emphasis) -> Self {
        Heading3Child::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for Heading3Child {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        Heading3Child::Emphasis(builder.build())
    }
}
impl From<Image> for Heading3Child {
    fn from(child: Image) -> Self {
        Heading3Child::Image(child)
    }
}
impl From<builders::ImageBuilder> for Heading3Child {
    fn from(builder: builders::ImageBuilder) -> Self {
        Heading3Child::Image(builder.build())
    }
}
impl From<InlineFrame> for Heading3Child {
    fn from(child: InlineFrame) -> Self {
        Heading3Child::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for Heading3Child {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        Heading3Child::InlineFrame(builder.build())
    }
}
impl From<Input> for Heading3Child {
    fn from(child: Input) -> Self {
        Heading3Child::Input(child)
    }
}
impl From<builders::InputBuilder> for Heading3Child {
    fn from(builder: builders::InputBuilder) -> Self {
        Heading3Child::Input(builder.build())
    }
}
impl From<Inserted> for Heading3Child {
    fn from(child: Inserted) -> Self {
        Heading3Child::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for Heading3Child {
    fn from(builder: builders::InsertedBuilder) -> Self {
        Heading3Child::Inserted(builder.build())
    }
}
impl From<Italic> for Heading3Child {
    fn from(child: Italic) -> Self {
        Heading3Child::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for Heading3Child {
    fn from(builder: builders::ItalicBuilder) -> Self {
        Heading3Child::Italic(builder.build())
    }
}
impl From<Keyboard> for Heading3Child {
    fn from(child: Keyboard) -> Self {
        Heading3Child::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for Heading3Child {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        Heading3Child::Keyboard(builder.build())
    }
}
impl From<Label> for Heading3Child {
    fn from(child: Label) -> Self {
        Heading3Child::Label(child)
    }
}
impl From<builders::LabelBuilder> for Heading3Child {
    fn from(builder: builders::LabelBuilder) -> Self {
        Heading3Child::Label(builder.build())
    }
}
impl From<LineBreak> for Heading3Child {
    fn from(child: LineBreak) -> Self {
        Heading3Child::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for Heading3Child {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        Heading3Child::LineBreak(builder.build())
    }
}
impl From<Link> for Heading3Child {
    fn from(child: Link) -> Self {
        Heading3Child::Link(child)
    }
}
impl From<builders::LinkBuilder> for Heading3Child {
    fn from(builder: builders::LinkBuilder) -> Self {
        Heading3Child::Link(builder.build())
    }
}
impl From<Map> for Heading3Child {
    fn from(child: Map) -> Self {
        Heading3Child::Map(child)
    }
}
impl From<builders::MapBuilder> for Heading3Child {
    fn from(builder: builders::MapBuilder) -> Self {
        Heading3Child::Map(builder.build())
    }
}
impl From<MapArea> for Heading3Child {
    fn from(child: MapArea) -> Self {
        Heading3Child::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for Heading3Child {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        Heading3Child::MapArea(builder.build())
    }
}
impl From<Mark> for Heading3Child {
    fn from(child: Mark) -> Self {
        Heading3Child::Mark(child)
    }
}
impl From<builders::MarkBuilder> for Heading3Child {
    fn from(builder: builders::MarkBuilder) -> Self {
        Heading3Child::Mark(builder.build())
    }
}
impl From<Metadata> for Heading3Child {
    fn from(child: Metadata) -> Self {
        Heading3Child::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for Heading3Child {
    fn from(builder: builders::MetadataBuilder) -> Self {
        Heading3Child::Metadata(builder.build())
    }
}
impl From<Meter> for Heading3Child {
    fn from(child: Meter) -> Self {
        Heading3Child::Meter(child)
    }
}
impl From<builders::MeterBuilder> for Heading3Child {
    fn from(builder: builders::MeterBuilder) -> Self {
        Heading3Child::Meter(builder.build())
    }
}
impl From<NoScript> for Heading3Child {
    fn from(child: NoScript) -> Self {
        Heading3Child::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for Heading3Child {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        Heading3Child::NoScript(builder.build())
    }
}
impl From<Object> for Heading3Child {
    fn from(child: Object) -> Self {
        Heading3Child::Object(child)
    }
}
impl From<builders::ObjectBuilder> for Heading3Child {
    fn from(builder: builders::ObjectBuilder) -> Self {
        Heading3Child::Object(builder.build())
    }
}
impl From<Output> for Heading3Child {
    fn from(child: Output) -> Self {
        Heading3Child::Output(child)
    }
}
impl From<builders::OutputBuilder> for Heading3Child {
    fn from(builder: builders::OutputBuilder) -> Self {
        Heading3Child::Output(builder.build())
    }
}
impl From<Picture> for Heading3Child {
    fn from(child: Picture) -> Self {
        Heading3Child::Picture(child)
    }
}
impl From<builders::PictureBuilder> for Heading3Child {
    fn from(builder: builders::PictureBuilder) -> Self {
        Heading3Child::Picture(builder.build())
    }
}
impl From<Progress> for Heading3Child {
    fn from(child: Progress) -> Self {
        Heading3Child::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for Heading3Child {
    fn from(builder: builders::ProgressBuilder) -> Self {
        Heading3Child::Progress(builder.build())
    }
}
impl From<Quote> for Heading3Child {
    fn from(child: Quote) -> Self {
        Heading3Child::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for Heading3Child {
    fn from(builder: builders::QuoteBuilder) -> Self {
        Heading3Child::Quote(builder.build())
    }
}
impl From<Ruby> for Heading3Child {
    fn from(child: Ruby) -> Self {
        Heading3Child::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for Heading3Child {
    fn from(builder: builders::RubyBuilder) -> Self {
        Heading3Child::Ruby(builder.build())
    }
}
impl From<Sample> for Heading3Child {
    fn from(child: Sample) -> Self {
        Heading3Child::Sample(child)
    }
}
impl From<builders::SampleBuilder> for Heading3Child {
    fn from(builder: builders::SampleBuilder) -> Self {
        Heading3Child::Sample(builder.build())
    }
}
impl From<Script> for Heading3Child {
    fn from(child: Script) -> Self {
        Heading3Child::Script(child)
    }
}
impl From<builders::ScriptBuilder> for Heading3Child {
    fn from(builder: builders::ScriptBuilder) -> Self {
        Heading3Child::Script(builder.build())
    }
}
impl From<Select> for Heading3Child {
    fn from(child: Select) -> Self {
        Heading3Child::Select(child)
    }
}
impl From<builders::SelectBuilder> for Heading3Child {
    fn from(builder: builders::SelectBuilder) -> Self {
        Heading3Child::Select(builder.build())
    }
}
impl From<Slot> for Heading3Child {
    fn from(child: Slot) -> Self {
        Heading3Child::Slot(child)
    }
}
impl From<builders::SlotBuilder> for Heading3Child {
    fn from(builder: builders::SlotBuilder) -> Self {
        Heading3Child::Slot(builder.build())
    }
}
impl From<Small> for Heading3Child {
    fn from(child: Small) -> Self {
        Heading3Child::Small(child)
    }
}
impl From<builders::SmallBuilder> for Heading3Child {
    fn from(builder: builders::SmallBuilder) -> Self {
        Heading3Child::Small(builder.build())
    }
}
impl From<Span> for Heading3Child {
    fn from(child: Span) -> Self {
        Heading3Child::Span(child)
    }
}
impl From<builders::SpanBuilder> for Heading3Child {
    fn from(builder: builders::SpanBuilder) -> Self {
        Heading3Child::Span(builder.build())
    }
}
impl From<StrikeThrough> for Heading3Child {
    fn from(child: StrikeThrough) -> Self {
        Heading3Child::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for Heading3Child {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        Heading3Child::StrikeThrough(builder.build())
    }
}
impl From<Strong> for Heading3Child {
    fn from(child: Strong) -> Self {
        Heading3Child::Strong(child)
    }
}
impl From<builders::StrongBuilder> for Heading3Child {
    fn from(builder: builders::StrongBuilder) -> Self {
        Heading3Child::Strong(builder.build())
    }
}
impl From<SubScript> for Heading3Child {
    fn from(child: SubScript) -> Self {
        Heading3Child::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for Heading3Child {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        Heading3Child::SubScript(builder.build())
    }
}
impl From<SupScript> for Heading3Child {
    fn from(child: SupScript) -> Self {
        Heading3Child::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for Heading3Child {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        Heading3Child::SupScript(builder.build())
    }
}
impl From<Template> for Heading3Child {
    fn from(child: Template) -> Self {
        Heading3Child::Template(child)
    }
}
impl From<builders::TemplateBuilder> for Heading3Child {
    fn from(builder: builders::TemplateBuilder) -> Self {
        Heading3Child::Template(builder.build())
    }
}
impl From<TextArea> for Heading3Child {
    fn from(child: TextArea) -> Self {
        Heading3Child::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for Heading3Child {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        Heading3Child::TextArea(builder.build())
    }
}
impl From<Time> for Heading3Child {
    fn from(child: Time) -> Self {
        Heading3Child::Time(child)
    }
}
impl From<builders::TimeBuilder> for Heading3Child {
    fn from(builder: builders::TimeBuilder) -> Self {
        Heading3Child::Time(builder.build())
    }
}
impl From<Underline> for Heading3Child {
    fn from(child: Underline) -> Self {
        Heading3Child::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for Heading3Child {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        Heading3Child::Underline(builder.build())
    }
}
impl From<Variable> for Heading3Child {
    fn from(child: Variable) -> Self {
        Heading3Child::Variable(child)
    }
}
impl From<builders::VariableBuilder> for Heading3Child {
    fn from(builder: builders::VariableBuilder) -> Self {
        Heading3Child::Variable(builder.build())
    }
}
impl From<Video> for Heading3Child {
    fn from(child: Video) -> Self {
        Heading3Child::Video(child)
    }
}
impl From<builders::VideoBuilder> for Heading3Child {
    fn from(builder: builders::VideoBuilder) -> Self {
        Heading3Child::Video(builder.build())
    }
}
impl From<WordBreak> for Heading3Child {
    fn from(child: WordBreak) -> Self {
        Heading3Child::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for Heading3Child {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        Heading3Child::WordBreak(builder.build())
    }
}
impl From<&'static str> for Heading3Child {
    fn from(s: &'static str) -> Self {
        Heading3Child::Text(s.into())
    }
}
impl From<String> for Heading3Child {
    fn from(s: String) -> Self {
        Heading3Child::Text(s.into())
    }
}
impl From<CowStr> for Heading3Child {
    fn from(s: CowStr) -> Self {
        Heading3Child::Text(s)
    }
}
impl std::fmt::Debug for Heading3Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heading3Child::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Anchor(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Audio(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Bold(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Button(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Canvas(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Cite(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Code(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Custom(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Data(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::DataList(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Definition(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Deleted(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Embed(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Image(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Input(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Inserted(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Italic(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Label(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Link(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Map(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::MapArea(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Mark(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Metadata(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Meter(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::NoScript(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Object(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Output(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Picture(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Progress(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Quote(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Ruby(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Sample(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Script(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Select(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Slot(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Small(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Span(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Strong(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::SubScript(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::SupScript(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Template(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::TextArea(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Time(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Underline(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Variable(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Video(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            Heading3Child::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for Heading3Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heading3Child::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Anchor(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Audio(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            Heading3Child::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            Heading3Child::Bold(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Button(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Canvas(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Cite(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Code(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Custom(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Data(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::DataList(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Definition(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Deleted(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Embed(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Emphasis(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Image(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Input(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Inserted(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Italic(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Keyboard(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Label(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::LineBreak(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Link(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Map(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::MapArea(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Mark(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Metadata(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Meter(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::NoScript(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Object(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Output(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Picture(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Progress(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Quote(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Ruby(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Sample(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Script(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Select(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Slot(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Small(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Span(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Strong(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::SubScript(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::SupScript(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Template(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::TextArea(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Time(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Underline(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Variable(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Video(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::WordBreak(child) => std::fmt::Display::fmt(child, f),
            Heading3Child::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
