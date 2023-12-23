// 🤖 This file is generated!

use crate::*;
/// The `<abbr>` element's children.
#[derive(Clone)]
pub enum AbbreviationChild {
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
impl From<Abbreviation> for AbbreviationChild {
    fn from(child: Abbreviation) -> Self {
        AbbreviationChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for AbbreviationChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        AbbreviationChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for AbbreviationChild {
    fn from(child: Anchor) -> Self {
        AbbreviationChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for AbbreviationChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        AbbreviationChild::Anchor(builder.build())
    }
}
impl From<Audio> for AbbreviationChild {
    fn from(child: Audio) -> Self {
        AbbreviationChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for AbbreviationChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        AbbreviationChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for AbbreviationChild {
    fn from(child: BidirectionalIsolate) -> Self {
        AbbreviationChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for AbbreviationChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        AbbreviationChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for AbbreviationChild {
    fn from(child: BidirectionalOverride) -> Self {
        AbbreviationChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for AbbreviationChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        AbbreviationChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for AbbreviationChild {
    fn from(child: Bold) -> Self {
        AbbreviationChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for AbbreviationChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        AbbreviationChild::Bold(builder.build())
    }
}
impl From<Button> for AbbreviationChild {
    fn from(child: Button) -> Self {
        AbbreviationChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for AbbreviationChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        AbbreviationChild::Button(builder.build())
    }
}
impl From<Canvas> for AbbreviationChild {
    fn from(child: Canvas) -> Self {
        AbbreviationChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for AbbreviationChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        AbbreviationChild::Canvas(builder.build())
    }
}
impl From<Cite> for AbbreviationChild {
    fn from(child: Cite) -> Self {
        AbbreviationChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for AbbreviationChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        AbbreviationChild::Cite(builder.build())
    }
}
impl From<Code> for AbbreviationChild {
    fn from(child: Code) -> Self {
        AbbreviationChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for AbbreviationChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        AbbreviationChild::Code(builder.build())
    }
}
impl From<Custom> for AbbreviationChild {
    fn from(child: Custom) -> Self {
        AbbreviationChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for AbbreviationChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        AbbreviationChild::Custom(builder.build())
    }
}
impl From<Data> for AbbreviationChild {
    fn from(child: Data) -> Self {
        AbbreviationChild::Data(child)
    }
}
impl From<builders::DataBuilder> for AbbreviationChild {
    fn from(builder: builders::DataBuilder) -> Self {
        AbbreviationChild::Data(builder.build())
    }
}
impl From<DataList> for AbbreviationChild {
    fn from(child: DataList) -> Self {
        AbbreviationChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for AbbreviationChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        AbbreviationChild::DataList(builder.build())
    }
}
impl From<Definition> for AbbreviationChild {
    fn from(child: Definition) -> Self {
        AbbreviationChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for AbbreviationChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        AbbreviationChild::Definition(builder.build())
    }
}
impl From<Deleted> for AbbreviationChild {
    fn from(child: Deleted) -> Self {
        AbbreviationChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for AbbreviationChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        AbbreviationChild::Deleted(builder.build())
    }
}
impl From<Embed> for AbbreviationChild {
    fn from(child: Embed) -> Self {
        AbbreviationChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for AbbreviationChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        AbbreviationChild::Embed(builder.build())
    }
}
impl From<Emphasis> for AbbreviationChild {
    fn from(child: Emphasis) -> Self {
        AbbreviationChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for AbbreviationChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        AbbreviationChild::Emphasis(builder.build())
    }
}
impl From<Image> for AbbreviationChild {
    fn from(child: Image) -> Self {
        AbbreviationChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for AbbreviationChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        AbbreviationChild::Image(builder.build())
    }
}
impl From<InlineFrame> for AbbreviationChild {
    fn from(child: InlineFrame) -> Self {
        AbbreviationChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for AbbreviationChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        AbbreviationChild::InlineFrame(builder.build())
    }
}
impl From<Input> for AbbreviationChild {
    fn from(child: Input) -> Self {
        AbbreviationChild::Input(child)
    }
}
impl From<builders::InputBuilder> for AbbreviationChild {
    fn from(builder: builders::InputBuilder) -> Self {
        AbbreviationChild::Input(builder.build())
    }
}
impl From<Inserted> for AbbreviationChild {
    fn from(child: Inserted) -> Self {
        AbbreviationChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for AbbreviationChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        AbbreviationChild::Inserted(builder.build())
    }
}
impl From<Italic> for AbbreviationChild {
    fn from(child: Italic) -> Self {
        AbbreviationChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for AbbreviationChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        AbbreviationChild::Italic(builder.build())
    }
}
impl From<Keyboard> for AbbreviationChild {
    fn from(child: Keyboard) -> Self {
        AbbreviationChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for AbbreviationChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        AbbreviationChild::Keyboard(builder.build())
    }
}
impl From<Label> for AbbreviationChild {
    fn from(child: Label) -> Self {
        AbbreviationChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for AbbreviationChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        AbbreviationChild::Label(builder.build())
    }
}
impl From<LineBreak> for AbbreviationChild {
    fn from(child: LineBreak) -> Self {
        AbbreviationChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for AbbreviationChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        AbbreviationChild::LineBreak(builder.build())
    }
}
impl From<Link> for AbbreviationChild {
    fn from(child: Link) -> Self {
        AbbreviationChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for AbbreviationChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        AbbreviationChild::Link(builder.build())
    }
}
impl From<Map> for AbbreviationChild {
    fn from(child: Map) -> Self {
        AbbreviationChild::Map(child)
    }
}
impl From<builders::MapBuilder> for AbbreviationChild {
    fn from(builder: builders::MapBuilder) -> Self {
        AbbreviationChild::Map(builder.build())
    }
}
impl From<MapArea> for AbbreviationChild {
    fn from(child: MapArea) -> Self {
        AbbreviationChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for AbbreviationChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        AbbreviationChild::MapArea(builder.build())
    }
}
impl From<Mark> for AbbreviationChild {
    fn from(child: Mark) -> Self {
        AbbreviationChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for AbbreviationChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        AbbreviationChild::Mark(builder.build())
    }
}
impl From<Metadata> for AbbreviationChild {
    fn from(child: Metadata) -> Self {
        AbbreviationChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for AbbreviationChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        AbbreviationChild::Metadata(builder.build())
    }
}
impl From<Meter> for AbbreviationChild {
    fn from(child: Meter) -> Self {
        AbbreviationChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for AbbreviationChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        AbbreviationChild::Meter(builder.build())
    }
}
impl From<NoScript> for AbbreviationChild {
    fn from(child: NoScript) -> Self {
        AbbreviationChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for AbbreviationChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        AbbreviationChild::NoScript(builder.build())
    }
}
impl From<Object> for AbbreviationChild {
    fn from(child: Object) -> Self {
        AbbreviationChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for AbbreviationChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        AbbreviationChild::Object(builder.build())
    }
}
impl From<Output> for AbbreviationChild {
    fn from(child: Output) -> Self {
        AbbreviationChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for AbbreviationChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        AbbreviationChild::Output(builder.build())
    }
}
impl From<Picture> for AbbreviationChild {
    fn from(child: Picture) -> Self {
        AbbreviationChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for AbbreviationChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        AbbreviationChild::Picture(builder.build())
    }
}
impl From<Progress> for AbbreviationChild {
    fn from(child: Progress) -> Self {
        AbbreviationChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for AbbreviationChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        AbbreviationChild::Progress(builder.build())
    }
}
impl From<Quote> for AbbreviationChild {
    fn from(child: Quote) -> Self {
        AbbreviationChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for AbbreviationChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        AbbreviationChild::Quote(builder.build())
    }
}
impl From<Ruby> for AbbreviationChild {
    fn from(child: Ruby) -> Self {
        AbbreviationChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for AbbreviationChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        AbbreviationChild::Ruby(builder.build())
    }
}
impl From<Sample> for AbbreviationChild {
    fn from(child: Sample) -> Self {
        AbbreviationChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for AbbreviationChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        AbbreviationChild::Sample(builder.build())
    }
}
impl From<Script> for AbbreviationChild {
    fn from(child: Script) -> Self {
        AbbreviationChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for AbbreviationChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        AbbreviationChild::Script(builder.build())
    }
}
impl From<Select> for AbbreviationChild {
    fn from(child: Select) -> Self {
        AbbreviationChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for AbbreviationChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        AbbreviationChild::Select(builder.build())
    }
}
impl From<Slot> for AbbreviationChild {
    fn from(child: Slot) -> Self {
        AbbreviationChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for AbbreviationChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        AbbreviationChild::Slot(builder.build())
    }
}
impl From<Small> for AbbreviationChild {
    fn from(child: Small) -> Self {
        AbbreviationChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for AbbreviationChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        AbbreviationChild::Small(builder.build())
    }
}
impl From<Span> for AbbreviationChild {
    fn from(child: Span) -> Self {
        AbbreviationChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for AbbreviationChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        AbbreviationChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for AbbreviationChild {
    fn from(child: StrikeThrough) -> Self {
        AbbreviationChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for AbbreviationChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        AbbreviationChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for AbbreviationChild {
    fn from(child: Strong) -> Self {
        AbbreviationChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for AbbreviationChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        AbbreviationChild::Strong(builder.build())
    }
}
impl From<SubScript> for AbbreviationChild {
    fn from(child: SubScript) -> Self {
        AbbreviationChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for AbbreviationChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        AbbreviationChild::SubScript(builder.build())
    }
}
impl From<SupScript> for AbbreviationChild {
    fn from(child: SupScript) -> Self {
        AbbreviationChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for AbbreviationChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        AbbreviationChild::SupScript(builder.build())
    }
}
impl From<Template> for AbbreviationChild {
    fn from(child: Template) -> Self {
        AbbreviationChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for AbbreviationChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        AbbreviationChild::Template(builder.build())
    }
}
impl From<TextArea> for AbbreviationChild {
    fn from(child: TextArea) -> Self {
        AbbreviationChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for AbbreviationChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        AbbreviationChild::TextArea(builder.build())
    }
}
impl From<Time> for AbbreviationChild {
    fn from(child: Time) -> Self {
        AbbreviationChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for AbbreviationChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        AbbreviationChild::Time(builder.build())
    }
}
impl From<Underline> for AbbreviationChild {
    fn from(child: Underline) -> Self {
        AbbreviationChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for AbbreviationChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        AbbreviationChild::Underline(builder.build())
    }
}
impl From<Variable> for AbbreviationChild {
    fn from(child: Variable) -> Self {
        AbbreviationChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for AbbreviationChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        AbbreviationChild::Variable(builder.build())
    }
}
impl From<Video> for AbbreviationChild {
    fn from(child: Video) -> Self {
        AbbreviationChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for AbbreviationChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        AbbreviationChild::Video(builder.build())
    }
}
impl From<WordBreak> for AbbreviationChild {
    fn from(child: WordBreak) -> Self {
        AbbreviationChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for AbbreviationChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        AbbreviationChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for AbbreviationChild {
    fn from(s: &'static str) -> Self {
        AbbreviationChild::Text(s.into())
    }
}
impl From<String> for AbbreviationChild {
    fn from(s: String) -> Self {
        AbbreviationChild::Text(s.into())
    }
}
impl From<CowStr> for AbbreviationChild {
    fn from(s: CowStr) -> Self {
        AbbreviationChild::Text(s)
    }
}
impl std::fmt::Debug for AbbreviationChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbbreviationChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::BidirectionalIsolate(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            AbbreviationChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            AbbreviationChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Button(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Code(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Data(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Image(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Input(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Label(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Link(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Map(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Object(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Output(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Script(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Select(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Small(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Span(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Template(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Time(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Video(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            AbbreviationChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for AbbreviationChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbbreviationChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Audio(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            AbbreviationChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            AbbreviationChild::Bold(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Button(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Cite(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Code(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Custom(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Data(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::DataList(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Definition(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Embed(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Image(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Input(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Italic(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Label(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Link(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Map(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Mark(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Meter(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Object(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Output(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Picture(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Progress(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Quote(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Sample(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Script(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Select(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Slot(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Small(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Span(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Strong(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Template(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Time(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Underline(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Variable(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Video(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            AbbreviationChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
