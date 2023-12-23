// 🤖 This file is generated!

use crate::*;
/// The `<header>` element's children.
#[derive(Clone)]
pub enum HeaderChild {
    Abbreviation(Abbreviation),
    Address(Address),
    Anchor(Anchor),
    Article(Article),
    Aside(Aside),
    Audio(Audio),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    BlockQuote(BlockQuote),
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
    DescriptionList(DescriptionList),
    Details(Details),
    Dialog(Dialog),
    Division(Division),
    Embed(Embed),
    Emphasis(Emphasis),
    FieldSet(FieldSet),
    Figure(Figure),
    Footer(Footer),
    Form(Form),
    Header(Header),
    Heading1(Heading1),
    Heading2(Heading2),
    Heading3(Heading3),
    Heading4(Heading4),
    Heading5(Heading5),
    Heading6(Heading6),
    HeadingGroup(HeadingGroup),
    HorizontalRule(HorizontalRule),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    Main(Main),
    Map(Map),
    MapArea(MapArea),
    Mark(Mark),
    Menu(Menu),
    Metadata(Metadata),
    Meter(Meter),
    Navigation(Navigation),
    NoScript(NoScript),
    Object(Object),
    OrderedList(OrderedList),
    Output(Output),
    Paragraph(Paragraph),
    Picture(Picture),
    Preformatted(Preformatted),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    Sample(Sample),
    Script(Script),
    Search(Search),
    Section(Section),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    SubScript(SubScript),
    SupScript(SupScript),
    Table(Table),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Underline(Underline),
    UnorderedList(UnorderedList),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
    Text(CowStr),
}
impl From<Abbreviation> for HeaderChild {
    fn from(child: Abbreviation) -> Self {
        HeaderChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for HeaderChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        HeaderChild::Abbreviation(builder.build())
    }
}
impl From<Address> for HeaderChild {
    fn from(child: Address) -> Self {
        HeaderChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for HeaderChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        HeaderChild::Address(builder.build())
    }
}
impl From<Anchor> for HeaderChild {
    fn from(child: Anchor) -> Self {
        HeaderChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for HeaderChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        HeaderChild::Anchor(builder.build())
    }
}
impl From<Article> for HeaderChild {
    fn from(child: Article) -> Self {
        HeaderChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for HeaderChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        HeaderChild::Article(builder.build())
    }
}
impl From<Aside> for HeaderChild {
    fn from(child: Aside) -> Self {
        HeaderChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for HeaderChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        HeaderChild::Aside(builder.build())
    }
}
impl From<Audio> for HeaderChild {
    fn from(child: Audio) -> Self {
        HeaderChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for HeaderChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        HeaderChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for HeaderChild {
    fn from(child: BidirectionalIsolate) -> Self {
        HeaderChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for HeaderChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        HeaderChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for HeaderChild {
    fn from(child: BidirectionalOverride) -> Self {
        HeaderChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for HeaderChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        HeaderChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for HeaderChild {
    fn from(child: BlockQuote) -> Self {
        HeaderChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for HeaderChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        HeaderChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for HeaderChild {
    fn from(child: Bold) -> Self {
        HeaderChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for HeaderChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        HeaderChild::Bold(builder.build())
    }
}
impl From<Button> for HeaderChild {
    fn from(child: Button) -> Self {
        HeaderChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for HeaderChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        HeaderChild::Button(builder.build())
    }
}
impl From<Canvas> for HeaderChild {
    fn from(child: Canvas) -> Self {
        HeaderChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for HeaderChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        HeaderChild::Canvas(builder.build())
    }
}
impl From<Cite> for HeaderChild {
    fn from(child: Cite) -> Self {
        HeaderChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for HeaderChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        HeaderChild::Cite(builder.build())
    }
}
impl From<Code> for HeaderChild {
    fn from(child: Code) -> Self {
        HeaderChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for HeaderChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        HeaderChild::Code(builder.build())
    }
}
impl From<Custom> for HeaderChild {
    fn from(child: Custom) -> Self {
        HeaderChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for HeaderChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        HeaderChild::Custom(builder.build())
    }
}
impl From<Data> for HeaderChild {
    fn from(child: Data) -> Self {
        HeaderChild::Data(child)
    }
}
impl From<builders::DataBuilder> for HeaderChild {
    fn from(builder: builders::DataBuilder) -> Self {
        HeaderChild::Data(builder.build())
    }
}
impl From<DataList> for HeaderChild {
    fn from(child: DataList) -> Self {
        HeaderChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for HeaderChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        HeaderChild::DataList(builder.build())
    }
}
impl From<Definition> for HeaderChild {
    fn from(child: Definition) -> Self {
        HeaderChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for HeaderChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        HeaderChild::Definition(builder.build())
    }
}
impl From<Deleted> for HeaderChild {
    fn from(child: Deleted) -> Self {
        HeaderChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for HeaderChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        HeaderChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for HeaderChild {
    fn from(child: DescriptionList) -> Self {
        HeaderChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for HeaderChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        HeaderChild::DescriptionList(builder.build())
    }
}
impl From<Details> for HeaderChild {
    fn from(child: Details) -> Self {
        HeaderChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for HeaderChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        HeaderChild::Details(builder.build())
    }
}
impl From<Dialog> for HeaderChild {
    fn from(child: Dialog) -> Self {
        HeaderChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for HeaderChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        HeaderChild::Dialog(builder.build())
    }
}
impl From<Division> for HeaderChild {
    fn from(child: Division) -> Self {
        HeaderChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for HeaderChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        HeaderChild::Division(builder.build())
    }
}
impl From<Embed> for HeaderChild {
    fn from(child: Embed) -> Self {
        HeaderChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for HeaderChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        HeaderChild::Embed(builder.build())
    }
}
impl From<Emphasis> for HeaderChild {
    fn from(child: Emphasis) -> Self {
        HeaderChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for HeaderChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        HeaderChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for HeaderChild {
    fn from(child: FieldSet) -> Self {
        HeaderChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for HeaderChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        HeaderChild::FieldSet(builder.build())
    }
}
impl From<Figure> for HeaderChild {
    fn from(child: Figure) -> Self {
        HeaderChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for HeaderChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        HeaderChild::Figure(builder.build())
    }
}
impl From<Footer> for HeaderChild {
    fn from(child: Footer) -> Self {
        HeaderChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for HeaderChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        HeaderChild::Footer(builder.build())
    }
}
impl From<Form> for HeaderChild {
    fn from(child: Form) -> Self {
        HeaderChild::Form(child)
    }
}
impl From<builders::FormBuilder> for HeaderChild {
    fn from(builder: builders::FormBuilder) -> Self {
        HeaderChild::Form(builder.build())
    }
}
impl From<Header> for HeaderChild {
    fn from(child: Header) -> Self {
        HeaderChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for HeaderChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        HeaderChild::Header(builder.build())
    }
}
impl From<Heading1> for HeaderChild {
    fn from(child: Heading1) -> Self {
        HeaderChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for HeaderChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        HeaderChild::Heading1(builder.build())
    }
}
impl From<Heading2> for HeaderChild {
    fn from(child: Heading2) -> Self {
        HeaderChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for HeaderChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        HeaderChild::Heading2(builder.build())
    }
}
impl From<Heading3> for HeaderChild {
    fn from(child: Heading3) -> Self {
        HeaderChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for HeaderChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        HeaderChild::Heading3(builder.build())
    }
}
impl From<Heading4> for HeaderChild {
    fn from(child: Heading4) -> Self {
        HeaderChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for HeaderChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        HeaderChild::Heading4(builder.build())
    }
}
impl From<Heading5> for HeaderChild {
    fn from(child: Heading5) -> Self {
        HeaderChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for HeaderChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        HeaderChild::Heading5(builder.build())
    }
}
impl From<Heading6> for HeaderChild {
    fn from(child: Heading6) -> Self {
        HeaderChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for HeaderChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        HeaderChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for HeaderChild {
    fn from(child: HeadingGroup) -> Self {
        HeaderChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for HeaderChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        HeaderChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for HeaderChild {
    fn from(child: HorizontalRule) -> Self {
        HeaderChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for HeaderChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        HeaderChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for HeaderChild {
    fn from(child: Image) -> Self {
        HeaderChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for HeaderChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        HeaderChild::Image(builder.build())
    }
}
impl From<InlineFrame> for HeaderChild {
    fn from(child: InlineFrame) -> Self {
        HeaderChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for HeaderChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        HeaderChild::InlineFrame(builder.build())
    }
}
impl From<Input> for HeaderChild {
    fn from(child: Input) -> Self {
        HeaderChild::Input(child)
    }
}
impl From<builders::InputBuilder> for HeaderChild {
    fn from(builder: builders::InputBuilder) -> Self {
        HeaderChild::Input(builder.build())
    }
}
impl From<Inserted> for HeaderChild {
    fn from(child: Inserted) -> Self {
        HeaderChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for HeaderChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        HeaderChild::Inserted(builder.build())
    }
}
impl From<Italic> for HeaderChild {
    fn from(child: Italic) -> Self {
        HeaderChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for HeaderChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        HeaderChild::Italic(builder.build())
    }
}
impl From<Keyboard> for HeaderChild {
    fn from(child: Keyboard) -> Self {
        HeaderChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for HeaderChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        HeaderChild::Keyboard(builder.build())
    }
}
impl From<Label> for HeaderChild {
    fn from(child: Label) -> Self {
        HeaderChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for HeaderChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        HeaderChild::Label(builder.build())
    }
}
impl From<LineBreak> for HeaderChild {
    fn from(child: LineBreak) -> Self {
        HeaderChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for HeaderChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        HeaderChild::LineBreak(builder.build())
    }
}
impl From<Link> for HeaderChild {
    fn from(child: Link) -> Self {
        HeaderChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for HeaderChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        HeaderChild::Link(builder.build())
    }
}
impl From<Main> for HeaderChild {
    fn from(child: Main) -> Self {
        HeaderChild::Main(child)
    }
}
impl From<builders::MainBuilder> for HeaderChild {
    fn from(builder: builders::MainBuilder) -> Self {
        HeaderChild::Main(builder.build())
    }
}
impl From<Map> for HeaderChild {
    fn from(child: Map) -> Self {
        HeaderChild::Map(child)
    }
}
impl From<builders::MapBuilder> for HeaderChild {
    fn from(builder: builders::MapBuilder) -> Self {
        HeaderChild::Map(builder.build())
    }
}
impl From<MapArea> for HeaderChild {
    fn from(child: MapArea) -> Self {
        HeaderChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for HeaderChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        HeaderChild::MapArea(builder.build())
    }
}
impl From<Mark> for HeaderChild {
    fn from(child: Mark) -> Self {
        HeaderChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for HeaderChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        HeaderChild::Mark(builder.build())
    }
}
impl From<Menu> for HeaderChild {
    fn from(child: Menu) -> Self {
        HeaderChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for HeaderChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        HeaderChild::Menu(builder.build())
    }
}
impl From<Metadata> for HeaderChild {
    fn from(child: Metadata) -> Self {
        HeaderChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for HeaderChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        HeaderChild::Metadata(builder.build())
    }
}
impl From<Meter> for HeaderChild {
    fn from(child: Meter) -> Self {
        HeaderChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for HeaderChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        HeaderChild::Meter(builder.build())
    }
}
impl From<Navigation> for HeaderChild {
    fn from(child: Navigation) -> Self {
        HeaderChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for HeaderChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        HeaderChild::Navigation(builder.build())
    }
}
impl From<NoScript> for HeaderChild {
    fn from(child: NoScript) -> Self {
        HeaderChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for HeaderChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        HeaderChild::NoScript(builder.build())
    }
}
impl From<Object> for HeaderChild {
    fn from(child: Object) -> Self {
        HeaderChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for HeaderChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        HeaderChild::Object(builder.build())
    }
}
impl From<OrderedList> for HeaderChild {
    fn from(child: OrderedList) -> Self {
        HeaderChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for HeaderChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        HeaderChild::OrderedList(builder.build())
    }
}
impl From<Output> for HeaderChild {
    fn from(child: Output) -> Self {
        HeaderChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for HeaderChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        HeaderChild::Output(builder.build())
    }
}
impl From<Paragraph> for HeaderChild {
    fn from(child: Paragraph) -> Self {
        HeaderChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for HeaderChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        HeaderChild::Paragraph(builder.build())
    }
}
impl From<Picture> for HeaderChild {
    fn from(child: Picture) -> Self {
        HeaderChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for HeaderChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        HeaderChild::Picture(builder.build())
    }
}
impl From<Preformatted> for HeaderChild {
    fn from(child: Preformatted) -> Self {
        HeaderChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for HeaderChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        HeaderChild::Preformatted(builder.build())
    }
}
impl From<Progress> for HeaderChild {
    fn from(child: Progress) -> Self {
        HeaderChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for HeaderChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        HeaderChild::Progress(builder.build())
    }
}
impl From<Quote> for HeaderChild {
    fn from(child: Quote) -> Self {
        HeaderChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for HeaderChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        HeaderChild::Quote(builder.build())
    }
}
impl From<Ruby> for HeaderChild {
    fn from(child: Ruby) -> Self {
        HeaderChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for HeaderChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        HeaderChild::Ruby(builder.build())
    }
}
impl From<Sample> for HeaderChild {
    fn from(child: Sample) -> Self {
        HeaderChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for HeaderChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        HeaderChild::Sample(builder.build())
    }
}
impl From<Script> for HeaderChild {
    fn from(child: Script) -> Self {
        HeaderChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for HeaderChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        HeaderChild::Script(builder.build())
    }
}
impl From<Search> for HeaderChild {
    fn from(child: Search) -> Self {
        HeaderChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for HeaderChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        HeaderChild::Search(builder.build())
    }
}
impl From<Section> for HeaderChild {
    fn from(child: Section) -> Self {
        HeaderChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for HeaderChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        HeaderChild::Section(builder.build())
    }
}
impl From<Select> for HeaderChild {
    fn from(child: Select) -> Self {
        HeaderChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for HeaderChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        HeaderChild::Select(builder.build())
    }
}
impl From<Slot> for HeaderChild {
    fn from(child: Slot) -> Self {
        HeaderChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for HeaderChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        HeaderChild::Slot(builder.build())
    }
}
impl From<Small> for HeaderChild {
    fn from(child: Small) -> Self {
        HeaderChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for HeaderChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        HeaderChild::Small(builder.build())
    }
}
impl From<Span> for HeaderChild {
    fn from(child: Span) -> Self {
        HeaderChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for HeaderChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        HeaderChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for HeaderChild {
    fn from(child: StrikeThrough) -> Self {
        HeaderChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for HeaderChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        HeaderChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for HeaderChild {
    fn from(child: Strong) -> Self {
        HeaderChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for HeaderChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        HeaderChild::Strong(builder.build())
    }
}
impl From<SubScript> for HeaderChild {
    fn from(child: SubScript) -> Self {
        HeaderChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for HeaderChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        HeaderChild::SubScript(builder.build())
    }
}
impl From<SupScript> for HeaderChild {
    fn from(child: SupScript) -> Self {
        HeaderChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for HeaderChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        HeaderChild::SupScript(builder.build())
    }
}
impl From<Table> for HeaderChild {
    fn from(child: Table) -> Self {
        HeaderChild::Table(child)
    }
}
impl From<builders::TableBuilder> for HeaderChild {
    fn from(builder: builders::TableBuilder) -> Self {
        HeaderChild::Table(builder.build())
    }
}
impl From<Template> for HeaderChild {
    fn from(child: Template) -> Self {
        HeaderChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for HeaderChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        HeaderChild::Template(builder.build())
    }
}
impl From<TextArea> for HeaderChild {
    fn from(child: TextArea) -> Self {
        HeaderChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for HeaderChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        HeaderChild::TextArea(builder.build())
    }
}
impl From<Time> for HeaderChild {
    fn from(child: Time) -> Self {
        HeaderChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for HeaderChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        HeaderChild::Time(builder.build())
    }
}
impl From<Underline> for HeaderChild {
    fn from(child: Underline) -> Self {
        HeaderChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for HeaderChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        HeaderChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for HeaderChild {
    fn from(child: UnorderedList) -> Self {
        HeaderChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for HeaderChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        HeaderChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for HeaderChild {
    fn from(child: Variable) -> Self {
        HeaderChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for HeaderChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        HeaderChild::Variable(builder.build())
    }
}
impl From<Video> for HeaderChild {
    fn from(child: Video) -> Self {
        HeaderChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for HeaderChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        HeaderChild::Video(builder.build())
    }
}
impl From<WordBreak> for HeaderChild {
    fn from(child: WordBreak) -> Self {
        HeaderChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for HeaderChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        HeaderChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for HeaderChild {
    fn from(s: &'static str) -> Self {
        HeaderChild::Text(s.into())
    }
}
impl From<String> for HeaderChild {
    fn from(s: String) -> Self {
        HeaderChild::Text(s.into())
    }
}
impl From<CowStr> for HeaderChild {
    fn from(s: CowStr) -> Self {
        HeaderChild::Text(s)
    }
}
impl std::fmt::Debug for HeaderChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeaderChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Address(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Article(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Button(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Code(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Data(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Details(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Division(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Form(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Header(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Image(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Input(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Label(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Link(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Main(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Map(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Object(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Output(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Script(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Search(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Section(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Select(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Small(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Span(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Table(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Template(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Time(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Video(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            HeaderChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for HeaderChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeaderChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Address(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Article(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Aside(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Audio(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Bold(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Button(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Cite(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Code(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Custom(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Data(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::DataList(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Definition(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Details(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Division(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Embed(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Figure(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Footer(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Form(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Header(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Image(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Input(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Italic(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Label(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Link(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Main(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Map(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Mark(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Menu(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Meter(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Object(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Output(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Picture(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Progress(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Quote(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Sample(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Script(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Search(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Section(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Select(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Slot(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Small(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Span(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Strong(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Table(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Template(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Time(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Underline(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Variable(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Video(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            HeaderChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
