// 🤖 This file is generated!

use crate::*;
/// The `<pre>` element's children.
#[derive(Clone)]
pub enum PreformattedChild {
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
impl From<Abbreviation> for PreformattedChild {
    fn from(child: Abbreviation) -> Self {
        PreformattedChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for PreformattedChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        PreformattedChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for PreformattedChild {
    fn from(child: Anchor) -> Self {
        PreformattedChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for PreformattedChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        PreformattedChild::Anchor(builder.build())
    }
}
impl From<Audio> for PreformattedChild {
    fn from(child: Audio) -> Self {
        PreformattedChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for PreformattedChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        PreformattedChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for PreformattedChild {
    fn from(child: BidirectionalIsolate) -> Self {
        PreformattedChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for PreformattedChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        PreformattedChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for PreformattedChild {
    fn from(child: BidirectionalOverride) -> Self {
        PreformattedChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for PreformattedChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        PreformattedChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for PreformattedChild {
    fn from(child: Bold) -> Self {
        PreformattedChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for PreformattedChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        PreformattedChild::Bold(builder.build())
    }
}
impl From<Button> for PreformattedChild {
    fn from(child: Button) -> Self {
        PreformattedChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for PreformattedChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        PreformattedChild::Button(builder.build())
    }
}
impl From<Canvas> for PreformattedChild {
    fn from(child: Canvas) -> Self {
        PreformattedChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for PreformattedChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        PreformattedChild::Canvas(builder.build())
    }
}
impl From<Cite> for PreformattedChild {
    fn from(child: Cite) -> Self {
        PreformattedChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for PreformattedChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        PreformattedChild::Cite(builder.build())
    }
}
impl From<Code> for PreformattedChild {
    fn from(child: Code) -> Self {
        PreformattedChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for PreformattedChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        PreformattedChild::Code(builder.build())
    }
}
impl From<Custom> for PreformattedChild {
    fn from(child: Custom) -> Self {
        PreformattedChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for PreformattedChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        PreformattedChild::Custom(builder.build())
    }
}
impl From<Data> for PreformattedChild {
    fn from(child: Data) -> Self {
        PreformattedChild::Data(child)
    }
}
impl From<builders::DataBuilder> for PreformattedChild {
    fn from(builder: builders::DataBuilder) -> Self {
        PreformattedChild::Data(builder.build())
    }
}
impl From<DataList> for PreformattedChild {
    fn from(child: DataList) -> Self {
        PreformattedChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for PreformattedChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        PreformattedChild::DataList(builder.build())
    }
}
impl From<Definition> for PreformattedChild {
    fn from(child: Definition) -> Self {
        PreformattedChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for PreformattedChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        PreformattedChild::Definition(builder.build())
    }
}
impl From<Deleted> for PreformattedChild {
    fn from(child: Deleted) -> Self {
        PreformattedChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for PreformattedChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        PreformattedChild::Deleted(builder.build())
    }
}
impl From<Embed> for PreformattedChild {
    fn from(child: Embed) -> Self {
        PreformattedChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for PreformattedChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        PreformattedChild::Embed(builder.build())
    }
}
impl From<Emphasis> for PreformattedChild {
    fn from(child: Emphasis) -> Self {
        PreformattedChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for PreformattedChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        PreformattedChild::Emphasis(builder.build())
    }
}
impl From<Image> for PreformattedChild {
    fn from(child: Image) -> Self {
        PreformattedChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for PreformattedChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        PreformattedChild::Image(builder.build())
    }
}
impl From<InlineFrame> for PreformattedChild {
    fn from(child: InlineFrame) -> Self {
        PreformattedChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for PreformattedChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        PreformattedChild::InlineFrame(builder.build())
    }
}
impl From<Input> for PreformattedChild {
    fn from(child: Input) -> Self {
        PreformattedChild::Input(child)
    }
}
impl From<builders::InputBuilder> for PreformattedChild {
    fn from(builder: builders::InputBuilder) -> Self {
        PreformattedChild::Input(builder.build())
    }
}
impl From<Inserted> for PreformattedChild {
    fn from(child: Inserted) -> Self {
        PreformattedChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for PreformattedChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        PreformattedChild::Inserted(builder.build())
    }
}
impl From<Italic> for PreformattedChild {
    fn from(child: Italic) -> Self {
        PreformattedChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for PreformattedChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        PreformattedChild::Italic(builder.build())
    }
}
impl From<Keyboard> for PreformattedChild {
    fn from(child: Keyboard) -> Self {
        PreformattedChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for PreformattedChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        PreformattedChild::Keyboard(builder.build())
    }
}
impl From<Label> for PreformattedChild {
    fn from(child: Label) -> Self {
        PreformattedChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for PreformattedChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        PreformattedChild::Label(builder.build())
    }
}
impl From<LineBreak> for PreformattedChild {
    fn from(child: LineBreak) -> Self {
        PreformattedChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for PreformattedChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        PreformattedChild::LineBreak(builder.build())
    }
}
impl From<Link> for PreformattedChild {
    fn from(child: Link) -> Self {
        PreformattedChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for PreformattedChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        PreformattedChild::Link(builder.build())
    }
}
impl From<Map> for PreformattedChild {
    fn from(child: Map) -> Self {
        PreformattedChild::Map(child)
    }
}
impl From<builders::MapBuilder> for PreformattedChild {
    fn from(builder: builders::MapBuilder) -> Self {
        PreformattedChild::Map(builder.build())
    }
}
impl From<MapArea> for PreformattedChild {
    fn from(child: MapArea) -> Self {
        PreformattedChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for PreformattedChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        PreformattedChild::MapArea(builder.build())
    }
}
impl From<Mark> for PreformattedChild {
    fn from(child: Mark) -> Self {
        PreformattedChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for PreformattedChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        PreformattedChild::Mark(builder.build())
    }
}
impl From<Metadata> for PreformattedChild {
    fn from(child: Metadata) -> Self {
        PreformattedChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for PreformattedChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        PreformattedChild::Metadata(builder.build())
    }
}
impl From<Meter> for PreformattedChild {
    fn from(child: Meter) -> Self {
        PreformattedChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for PreformattedChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        PreformattedChild::Meter(builder.build())
    }
}
impl From<NoScript> for PreformattedChild {
    fn from(child: NoScript) -> Self {
        PreformattedChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for PreformattedChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        PreformattedChild::NoScript(builder.build())
    }
}
impl From<Object> for PreformattedChild {
    fn from(child: Object) -> Self {
        PreformattedChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for PreformattedChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        PreformattedChild::Object(builder.build())
    }
}
impl From<Output> for PreformattedChild {
    fn from(child: Output) -> Self {
        PreformattedChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for PreformattedChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        PreformattedChild::Output(builder.build())
    }
}
impl From<Picture> for PreformattedChild {
    fn from(child: Picture) -> Self {
        PreformattedChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for PreformattedChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        PreformattedChild::Picture(builder.build())
    }
}
impl From<Progress> for PreformattedChild {
    fn from(child: Progress) -> Self {
        PreformattedChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for PreformattedChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        PreformattedChild::Progress(builder.build())
    }
}
impl From<Quote> for PreformattedChild {
    fn from(child: Quote) -> Self {
        PreformattedChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for PreformattedChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        PreformattedChild::Quote(builder.build())
    }
}
impl From<Ruby> for PreformattedChild {
    fn from(child: Ruby) -> Self {
        PreformattedChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for PreformattedChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        PreformattedChild::Ruby(builder.build())
    }
}
impl From<Sample> for PreformattedChild {
    fn from(child: Sample) -> Self {
        PreformattedChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for PreformattedChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        PreformattedChild::Sample(builder.build())
    }
}
impl From<Script> for PreformattedChild {
    fn from(child: Script) -> Self {
        PreformattedChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for PreformattedChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        PreformattedChild::Script(builder.build())
    }
}
impl From<Select> for PreformattedChild {
    fn from(child: Select) -> Self {
        PreformattedChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for PreformattedChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        PreformattedChild::Select(builder.build())
    }
}
impl From<Slot> for PreformattedChild {
    fn from(child: Slot) -> Self {
        PreformattedChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for PreformattedChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        PreformattedChild::Slot(builder.build())
    }
}
impl From<Small> for PreformattedChild {
    fn from(child: Small) -> Self {
        PreformattedChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for PreformattedChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        PreformattedChild::Small(builder.build())
    }
}
impl From<Span> for PreformattedChild {
    fn from(child: Span) -> Self {
        PreformattedChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for PreformattedChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        PreformattedChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for PreformattedChild {
    fn from(child: StrikeThrough) -> Self {
        PreformattedChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for PreformattedChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        PreformattedChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for PreformattedChild {
    fn from(child: Strong) -> Self {
        PreformattedChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for PreformattedChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        PreformattedChild::Strong(builder.build())
    }
}
impl From<SubScript> for PreformattedChild {
    fn from(child: SubScript) -> Self {
        PreformattedChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for PreformattedChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        PreformattedChild::SubScript(builder.build())
    }
}
impl From<SupScript> for PreformattedChild {
    fn from(child: SupScript) -> Self {
        PreformattedChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for PreformattedChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        PreformattedChild::SupScript(builder.build())
    }
}
impl From<Template> for PreformattedChild {
    fn from(child: Template) -> Self {
        PreformattedChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for PreformattedChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        PreformattedChild::Template(builder.build())
    }
}
impl From<TextArea> for PreformattedChild {
    fn from(child: TextArea) -> Self {
        PreformattedChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for PreformattedChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        PreformattedChild::TextArea(builder.build())
    }
}
impl From<Time> for PreformattedChild {
    fn from(child: Time) -> Self {
        PreformattedChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for PreformattedChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        PreformattedChild::Time(builder.build())
    }
}
impl From<Underline> for PreformattedChild {
    fn from(child: Underline) -> Self {
        PreformattedChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for PreformattedChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        PreformattedChild::Underline(builder.build())
    }
}
impl From<Variable> for PreformattedChild {
    fn from(child: Variable) -> Self {
        PreformattedChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for PreformattedChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        PreformattedChild::Variable(builder.build())
    }
}
impl From<Video> for PreformattedChild {
    fn from(child: Video) -> Self {
        PreformattedChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for PreformattedChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        PreformattedChild::Video(builder.build())
    }
}
impl From<WordBreak> for PreformattedChild {
    fn from(child: WordBreak) -> Self {
        PreformattedChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for PreformattedChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        PreformattedChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for PreformattedChild {
    fn from(s: &'static str) -> Self {
        PreformattedChild::Text(s.into())
    }
}
impl From<String> for PreformattedChild {
    fn from(s: String) -> Self {
        PreformattedChild::Text(s.into())
    }
}
impl From<CowStr> for PreformattedChild {
    fn from(s: CowStr) -> Self {
        PreformattedChild::Text(s)
    }
}
impl std::fmt::Debug for PreformattedChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreformattedChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::BidirectionalIsolate(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            PreformattedChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            PreformattedChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Button(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Code(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Data(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Image(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Input(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Label(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Link(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Map(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Object(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Output(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Script(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Select(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Small(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Span(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Template(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Time(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Video(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            PreformattedChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for PreformattedChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreformattedChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Audio(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            PreformattedChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            PreformattedChild::Bold(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Button(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Cite(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Code(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Custom(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Data(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::DataList(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Definition(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Embed(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Image(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Input(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Italic(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Label(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Link(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Map(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Mark(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Meter(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Object(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Output(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Picture(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Progress(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Quote(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Sample(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Script(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Select(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Slot(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Small(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Span(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Strong(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Template(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Time(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Underline(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Variable(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Video(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            PreformattedChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
