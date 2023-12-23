// 🤖 This file is generated!

use crate::*;
/// The `<datalist>` element's children.
#[derive(Clone)]
pub enum DataListChild {
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
    Option(Option),
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
impl From<Abbreviation> for DataListChild {
    fn from(child: Abbreviation) -> Self {
        DataListChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for DataListChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        DataListChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for DataListChild {
    fn from(child: Anchor) -> Self {
        DataListChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for DataListChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        DataListChild::Anchor(builder.build())
    }
}
impl From<Audio> for DataListChild {
    fn from(child: Audio) -> Self {
        DataListChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for DataListChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        DataListChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for DataListChild {
    fn from(child: BidirectionalIsolate) -> Self {
        DataListChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for DataListChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        DataListChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for DataListChild {
    fn from(child: BidirectionalOverride) -> Self {
        DataListChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for DataListChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        DataListChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for DataListChild {
    fn from(child: Bold) -> Self {
        DataListChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for DataListChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        DataListChild::Bold(builder.build())
    }
}
impl From<Button> for DataListChild {
    fn from(child: Button) -> Self {
        DataListChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for DataListChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        DataListChild::Button(builder.build())
    }
}
impl From<Canvas> for DataListChild {
    fn from(child: Canvas) -> Self {
        DataListChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for DataListChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        DataListChild::Canvas(builder.build())
    }
}
impl From<Cite> for DataListChild {
    fn from(child: Cite) -> Self {
        DataListChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for DataListChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        DataListChild::Cite(builder.build())
    }
}
impl From<Code> for DataListChild {
    fn from(child: Code) -> Self {
        DataListChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for DataListChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        DataListChild::Code(builder.build())
    }
}
impl From<Custom> for DataListChild {
    fn from(child: Custom) -> Self {
        DataListChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for DataListChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        DataListChild::Custom(builder.build())
    }
}
impl From<Data> for DataListChild {
    fn from(child: Data) -> Self {
        DataListChild::Data(child)
    }
}
impl From<builders::DataBuilder> for DataListChild {
    fn from(builder: builders::DataBuilder) -> Self {
        DataListChild::Data(builder.build())
    }
}
impl From<DataList> for DataListChild {
    fn from(child: DataList) -> Self {
        DataListChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for DataListChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        DataListChild::DataList(builder.build())
    }
}
impl From<Definition> for DataListChild {
    fn from(child: Definition) -> Self {
        DataListChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for DataListChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        DataListChild::Definition(builder.build())
    }
}
impl From<Deleted> for DataListChild {
    fn from(child: Deleted) -> Self {
        DataListChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for DataListChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        DataListChild::Deleted(builder.build())
    }
}
impl From<Embed> for DataListChild {
    fn from(child: Embed) -> Self {
        DataListChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for DataListChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        DataListChild::Embed(builder.build())
    }
}
impl From<Emphasis> for DataListChild {
    fn from(child: Emphasis) -> Self {
        DataListChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for DataListChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        DataListChild::Emphasis(builder.build())
    }
}
impl From<Image> for DataListChild {
    fn from(child: Image) -> Self {
        DataListChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for DataListChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        DataListChild::Image(builder.build())
    }
}
impl From<InlineFrame> for DataListChild {
    fn from(child: InlineFrame) -> Self {
        DataListChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for DataListChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        DataListChild::InlineFrame(builder.build())
    }
}
impl From<Input> for DataListChild {
    fn from(child: Input) -> Self {
        DataListChild::Input(child)
    }
}
impl From<builders::InputBuilder> for DataListChild {
    fn from(builder: builders::InputBuilder) -> Self {
        DataListChild::Input(builder.build())
    }
}
impl From<Inserted> for DataListChild {
    fn from(child: Inserted) -> Self {
        DataListChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for DataListChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        DataListChild::Inserted(builder.build())
    }
}
impl From<Italic> for DataListChild {
    fn from(child: Italic) -> Self {
        DataListChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for DataListChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        DataListChild::Italic(builder.build())
    }
}
impl From<Keyboard> for DataListChild {
    fn from(child: Keyboard) -> Self {
        DataListChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for DataListChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        DataListChild::Keyboard(builder.build())
    }
}
impl From<Label> for DataListChild {
    fn from(child: Label) -> Self {
        DataListChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for DataListChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        DataListChild::Label(builder.build())
    }
}
impl From<LineBreak> for DataListChild {
    fn from(child: LineBreak) -> Self {
        DataListChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for DataListChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        DataListChild::LineBreak(builder.build())
    }
}
impl From<Link> for DataListChild {
    fn from(child: Link) -> Self {
        DataListChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for DataListChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        DataListChild::Link(builder.build())
    }
}
impl From<Map> for DataListChild {
    fn from(child: Map) -> Self {
        DataListChild::Map(child)
    }
}
impl From<builders::MapBuilder> for DataListChild {
    fn from(builder: builders::MapBuilder) -> Self {
        DataListChild::Map(builder.build())
    }
}
impl From<MapArea> for DataListChild {
    fn from(child: MapArea) -> Self {
        DataListChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for DataListChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        DataListChild::MapArea(builder.build())
    }
}
impl From<Mark> for DataListChild {
    fn from(child: Mark) -> Self {
        DataListChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for DataListChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        DataListChild::Mark(builder.build())
    }
}
impl From<Metadata> for DataListChild {
    fn from(child: Metadata) -> Self {
        DataListChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for DataListChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        DataListChild::Metadata(builder.build())
    }
}
impl From<Meter> for DataListChild {
    fn from(child: Meter) -> Self {
        DataListChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for DataListChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        DataListChild::Meter(builder.build())
    }
}
impl From<NoScript> for DataListChild {
    fn from(child: NoScript) -> Self {
        DataListChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for DataListChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        DataListChild::NoScript(builder.build())
    }
}
impl From<Object> for DataListChild {
    fn from(child: Object) -> Self {
        DataListChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for DataListChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        DataListChild::Object(builder.build())
    }
}
impl From<Option> for DataListChild {
    fn from(child: Option) -> Self {
        DataListChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for DataListChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        DataListChild::Option(builder.build())
    }
}
impl From<Output> for DataListChild {
    fn from(child: Output) -> Self {
        DataListChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for DataListChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        DataListChild::Output(builder.build())
    }
}
impl From<Picture> for DataListChild {
    fn from(child: Picture) -> Self {
        DataListChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for DataListChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        DataListChild::Picture(builder.build())
    }
}
impl From<Progress> for DataListChild {
    fn from(child: Progress) -> Self {
        DataListChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for DataListChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        DataListChild::Progress(builder.build())
    }
}
impl From<Quote> for DataListChild {
    fn from(child: Quote) -> Self {
        DataListChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for DataListChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        DataListChild::Quote(builder.build())
    }
}
impl From<Ruby> for DataListChild {
    fn from(child: Ruby) -> Self {
        DataListChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for DataListChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        DataListChild::Ruby(builder.build())
    }
}
impl From<Sample> for DataListChild {
    fn from(child: Sample) -> Self {
        DataListChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for DataListChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        DataListChild::Sample(builder.build())
    }
}
impl From<Script> for DataListChild {
    fn from(child: Script) -> Self {
        DataListChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for DataListChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        DataListChild::Script(builder.build())
    }
}
impl From<Select> for DataListChild {
    fn from(child: Select) -> Self {
        DataListChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for DataListChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        DataListChild::Select(builder.build())
    }
}
impl From<Slot> for DataListChild {
    fn from(child: Slot) -> Self {
        DataListChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for DataListChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        DataListChild::Slot(builder.build())
    }
}
impl From<Small> for DataListChild {
    fn from(child: Small) -> Self {
        DataListChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for DataListChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        DataListChild::Small(builder.build())
    }
}
impl From<Span> for DataListChild {
    fn from(child: Span) -> Self {
        DataListChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for DataListChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        DataListChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for DataListChild {
    fn from(child: StrikeThrough) -> Self {
        DataListChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for DataListChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        DataListChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for DataListChild {
    fn from(child: Strong) -> Self {
        DataListChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for DataListChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        DataListChild::Strong(builder.build())
    }
}
impl From<SubScript> for DataListChild {
    fn from(child: SubScript) -> Self {
        DataListChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for DataListChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        DataListChild::SubScript(builder.build())
    }
}
impl From<SupScript> for DataListChild {
    fn from(child: SupScript) -> Self {
        DataListChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for DataListChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        DataListChild::SupScript(builder.build())
    }
}
impl From<Template> for DataListChild {
    fn from(child: Template) -> Self {
        DataListChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for DataListChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        DataListChild::Template(builder.build())
    }
}
impl From<TextArea> for DataListChild {
    fn from(child: TextArea) -> Self {
        DataListChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for DataListChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        DataListChild::TextArea(builder.build())
    }
}
impl From<Time> for DataListChild {
    fn from(child: Time) -> Self {
        DataListChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for DataListChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        DataListChild::Time(builder.build())
    }
}
impl From<Underline> for DataListChild {
    fn from(child: Underline) -> Self {
        DataListChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for DataListChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        DataListChild::Underline(builder.build())
    }
}
impl From<Variable> for DataListChild {
    fn from(child: Variable) -> Self {
        DataListChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for DataListChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        DataListChild::Variable(builder.build())
    }
}
impl From<Video> for DataListChild {
    fn from(child: Video) -> Self {
        DataListChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for DataListChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        DataListChild::Video(builder.build())
    }
}
impl From<WordBreak> for DataListChild {
    fn from(child: WordBreak) -> Self {
        DataListChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for DataListChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        DataListChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for DataListChild {
    fn from(s: &'static str) -> Self {
        DataListChild::Text(s.into())
    }
}
impl From<String> for DataListChild {
    fn from(s: String) -> Self {
        DataListChild::Text(s.into())
    }
}
impl From<CowStr> for DataListChild {
    fn from(s: CowStr) -> Self {
        DataListChild::Text(s)
    }
}
impl std::fmt::Debug for DataListChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataListChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Button(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Code(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Data(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Image(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Input(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Label(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Link(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Map(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Object(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Option(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Output(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Script(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Select(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Small(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Span(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Template(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Time(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Video(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            DataListChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for DataListChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataListChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Audio(child) => std::fmt::Display::fmt(child, f),
            DataListChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DataListChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DataListChild::Bold(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Button(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Cite(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Code(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Custom(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Data(child) => std::fmt::Display::fmt(child, f),
            DataListChild::DataList(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Definition(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Embed(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Image(child) => std::fmt::Display::fmt(child, f),
            DataListChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Input(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Italic(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Label(child) => std::fmt::Display::fmt(child, f),
            DataListChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Link(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Map(child) => std::fmt::Display::fmt(child, f),
            DataListChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Mark(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Meter(child) => std::fmt::Display::fmt(child, f),
            DataListChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Object(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Option(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Output(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Picture(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Progress(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Quote(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Sample(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Script(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Select(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Slot(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Small(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Span(child) => std::fmt::Display::fmt(child, f),
            DataListChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Strong(child) => std::fmt::Display::fmt(child, f),
            DataListChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            DataListChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Template(child) => std::fmt::Display::fmt(child, f),
            DataListChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Time(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Underline(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Variable(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Video(child) => std::fmt::Display::fmt(child, f),
            DataListChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            DataListChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
