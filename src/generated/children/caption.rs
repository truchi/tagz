// 🤖 This file is generated!

use crate::*;
/// The `<caption>` element's children.
#[derive(Clone)]
pub enum CaptionChild {
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
impl From<Abbreviation> for CaptionChild {
    fn from(child: Abbreviation) -> Self {
        CaptionChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for CaptionChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        CaptionChild::Abbreviation(builder.build())
    }
}
impl From<Address> for CaptionChild {
    fn from(child: Address) -> Self {
        CaptionChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for CaptionChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        CaptionChild::Address(builder.build())
    }
}
impl From<Anchor> for CaptionChild {
    fn from(child: Anchor) -> Self {
        CaptionChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for CaptionChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        CaptionChild::Anchor(builder.build())
    }
}
impl From<Article> for CaptionChild {
    fn from(child: Article) -> Self {
        CaptionChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for CaptionChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        CaptionChild::Article(builder.build())
    }
}
impl From<Aside> for CaptionChild {
    fn from(child: Aside) -> Self {
        CaptionChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for CaptionChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        CaptionChild::Aside(builder.build())
    }
}
impl From<Audio> for CaptionChild {
    fn from(child: Audio) -> Self {
        CaptionChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for CaptionChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        CaptionChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for CaptionChild {
    fn from(child: BidirectionalIsolate) -> Self {
        CaptionChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for CaptionChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        CaptionChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for CaptionChild {
    fn from(child: BidirectionalOverride) -> Self {
        CaptionChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for CaptionChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        CaptionChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for CaptionChild {
    fn from(child: BlockQuote) -> Self {
        CaptionChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for CaptionChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        CaptionChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for CaptionChild {
    fn from(child: Bold) -> Self {
        CaptionChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for CaptionChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        CaptionChild::Bold(builder.build())
    }
}
impl From<Button> for CaptionChild {
    fn from(child: Button) -> Self {
        CaptionChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for CaptionChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        CaptionChild::Button(builder.build())
    }
}
impl From<Canvas> for CaptionChild {
    fn from(child: Canvas) -> Self {
        CaptionChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for CaptionChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        CaptionChild::Canvas(builder.build())
    }
}
impl From<Cite> for CaptionChild {
    fn from(child: Cite) -> Self {
        CaptionChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for CaptionChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        CaptionChild::Cite(builder.build())
    }
}
impl From<Code> for CaptionChild {
    fn from(child: Code) -> Self {
        CaptionChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for CaptionChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        CaptionChild::Code(builder.build())
    }
}
impl From<Custom> for CaptionChild {
    fn from(child: Custom) -> Self {
        CaptionChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for CaptionChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        CaptionChild::Custom(builder.build())
    }
}
impl From<Data> for CaptionChild {
    fn from(child: Data) -> Self {
        CaptionChild::Data(child)
    }
}
impl From<builders::DataBuilder> for CaptionChild {
    fn from(builder: builders::DataBuilder) -> Self {
        CaptionChild::Data(builder.build())
    }
}
impl From<DataList> for CaptionChild {
    fn from(child: DataList) -> Self {
        CaptionChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for CaptionChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        CaptionChild::DataList(builder.build())
    }
}
impl From<Definition> for CaptionChild {
    fn from(child: Definition) -> Self {
        CaptionChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for CaptionChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        CaptionChild::Definition(builder.build())
    }
}
impl From<Deleted> for CaptionChild {
    fn from(child: Deleted) -> Self {
        CaptionChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for CaptionChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        CaptionChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for CaptionChild {
    fn from(child: DescriptionList) -> Self {
        CaptionChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for CaptionChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        CaptionChild::DescriptionList(builder.build())
    }
}
impl From<Details> for CaptionChild {
    fn from(child: Details) -> Self {
        CaptionChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for CaptionChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        CaptionChild::Details(builder.build())
    }
}
impl From<Dialog> for CaptionChild {
    fn from(child: Dialog) -> Self {
        CaptionChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for CaptionChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        CaptionChild::Dialog(builder.build())
    }
}
impl From<Division> for CaptionChild {
    fn from(child: Division) -> Self {
        CaptionChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for CaptionChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        CaptionChild::Division(builder.build())
    }
}
impl From<Embed> for CaptionChild {
    fn from(child: Embed) -> Self {
        CaptionChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for CaptionChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        CaptionChild::Embed(builder.build())
    }
}
impl From<Emphasis> for CaptionChild {
    fn from(child: Emphasis) -> Self {
        CaptionChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for CaptionChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        CaptionChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for CaptionChild {
    fn from(child: FieldSet) -> Self {
        CaptionChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for CaptionChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        CaptionChild::FieldSet(builder.build())
    }
}
impl From<Figure> for CaptionChild {
    fn from(child: Figure) -> Self {
        CaptionChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for CaptionChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        CaptionChild::Figure(builder.build())
    }
}
impl From<Footer> for CaptionChild {
    fn from(child: Footer) -> Self {
        CaptionChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for CaptionChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        CaptionChild::Footer(builder.build())
    }
}
impl From<Form> for CaptionChild {
    fn from(child: Form) -> Self {
        CaptionChild::Form(child)
    }
}
impl From<builders::FormBuilder> for CaptionChild {
    fn from(builder: builders::FormBuilder) -> Self {
        CaptionChild::Form(builder.build())
    }
}
impl From<Header> for CaptionChild {
    fn from(child: Header) -> Self {
        CaptionChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for CaptionChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        CaptionChild::Header(builder.build())
    }
}
impl From<Heading1> for CaptionChild {
    fn from(child: Heading1) -> Self {
        CaptionChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for CaptionChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        CaptionChild::Heading1(builder.build())
    }
}
impl From<Heading2> for CaptionChild {
    fn from(child: Heading2) -> Self {
        CaptionChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for CaptionChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        CaptionChild::Heading2(builder.build())
    }
}
impl From<Heading3> for CaptionChild {
    fn from(child: Heading3) -> Self {
        CaptionChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for CaptionChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        CaptionChild::Heading3(builder.build())
    }
}
impl From<Heading4> for CaptionChild {
    fn from(child: Heading4) -> Self {
        CaptionChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for CaptionChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        CaptionChild::Heading4(builder.build())
    }
}
impl From<Heading5> for CaptionChild {
    fn from(child: Heading5) -> Self {
        CaptionChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for CaptionChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        CaptionChild::Heading5(builder.build())
    }
}
impl From<Heading6> for CaptionChild {
    fn from(child: Heading6) -> Self {
        CaptionChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for CaptionChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        CaptionChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for CaptionChild {
    fn from(child: HeadingGroup) -> Self {
        CaptionChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for CaptionChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        CaptionChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for CaptionChild {
    fn from(child: HorizontalRule) -> Self {
        CaptionChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for CaptionChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        CaptionChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for CaptionChild {
    fn from(child: Image) -> Self {
        CaptionChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for CaptionChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        CaptionChild::Image(builder.build())
    }
}
impl From<InlineFrame> for CaptionChild {
    fn from(child: InlineFrame) -> Self {
        CaptionChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for CaptionChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        CaptionChild::InlineFrame(builder.build())
    }
}
impl From<Input> for CaptionChild {
    fn from(child: Input) -> Self {
        CaptionChild::Input(child)
    }
}
impl From<builders::InputBuilder> for CaptionChild {
    fn from(builder: builders::InputBuilder) -> Self {
        CaptionChild::Input(builder.build())
    }
}
impl From<Inserted> for CaptionChild {
    fn from(child: Inserted) -> Self {
        CaptionChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for CaptionChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        CaptionChild::Inserted(builder.build())
    }
}
impl From<Italic> for CaptionChild {
    fn from(child: Italic) -> Self {
        CaptionChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for CaptionChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        CaptionChild::Italic(builder.build())
    }
}
impl From<Keyboard> for CaptionChild {
    fn from(child: Keyboard) -> Self {
        CaptionChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for CaptionChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        CaptionChild::Keyboard(builder.build())
    }
}
impl From<Label> for CaptionChild {
    fn from(child: Label) -> Self {
        CaptionChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for CaptionChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        CaptionChild::Label(builder.build())
    }
}
impl From<LineBreak> for CaptionChild {
    fn from(child: LineBreak) -> Self {
        CaptionChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for CaptionChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        CaptionChild::LineBreak(builder.build())
    }
}
impl From<Link> for CaptionChild {
    fn from(child: Link) -> Self {
        CaptionChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for CaptionChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        CaptionChild::Link(builder.build())
    }
}
impl From<Main> for CaptionChild {
    fn from(child: Main) -> Self {
        CaptionChild::Main(child)
    }
}
impl From<builders::MainBuilder> for CaptionChild {
    fn from(builder: builders::MainBuilder) -> Self {
        CaptionChild::Main(builder.build())
    }
}
impl From<Map> for CaptionChild {
    fn from(child: Map) -> Self {
        CaptionChild::Map(child)
    }
}
impl From<builders::MapBuilder> for CaptionChild {
    fn from(builder: builders::MapBuilder) -> Self {
        CaptionChild::Map(builder.build())
    }
}
impl From<MapArea> for CaptionChild {
    fn from(child: MapArea) -> Self {
        CaptionChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for CaptionChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        CaptionChild::MapArea(builder.build())
    }
}
impl From<Mark> for CaptionChild {
    fn from(child: Mark) -> Self {
        CaptionChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for CaptionChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        CaptionChild::Mark(builder.build())
    }
}
impl From<Menu> for CaptionChild {
    fn from(child: Menu) -> Self {
        CaptionChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for CaptionChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        CaptionChild::Menu(builder.build())
    }
}
impl From<Metadata> for CaptionChild {
    fn from(child: Metadata) -> Self {
        CaptionChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for CaptionChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        CaptionChild::Metadata(builder.build())
    }
}
impl From<Meter> for CaptionChild {
    fn from(child: Meter) -> Self {
        CaptionChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for CaptionChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        CaptionChild::Meter(builder.build())
    }
}
impl From<Navigation> for CaptionChild {
    fn from(child: Navigation) -> Self {
        CaptionChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for CaptionChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        CaptionChild::Navigation(builder.build())
    }
}
impl From<NoScript> for CaptionChild {
    fn from(child: NoScript) -> Self {
        CaptionChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for CaptionChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        CaptionChild::NoScript(builder.build())
    }
}
impl From<Object> for CaptionChild {
    fn from(child: Object) -> Self {
        CaptionChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for CaptionChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        CaptionChild::Object(builder.build())
    }
}
impl From<OrderedList> for CaptionChild {
    fn from(child: OrderedList) -> Self {
        CaptionChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for CaptionChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        CaptionChild::OrderedList(builder.build())
    }
}
impl From<Output> for CaptionChild {
    fn from(child: Output) -> Self {
        CaptionChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for CaptionChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        CaptionChild::Output(builder.build())
    }
}
impl From<Paragraph> for CaptionChild {
    fn from(child: Paragraph) -> Self {
        CaptionChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for CaptionChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        CaptionChild::Paragraph(builder.build())
    }
}
impl From<Picture> for CaptionChild {
    fn from(child: Picture) -> Self {
        CaptionChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for CaptionChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        CaptionChild::Picture(builder.build())
    }
}
impl From<Preformatted> for CaptionChild {
    fn from(child: Preformatted) -> Self {
        CaptionChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for CaptionChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        CaptionChild::Preformatted(builder.build())
    }
}
impl From<Progress> for CaptionChild {
    fn from(child: Progress) -> Self {
        CaptionChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for CaptionChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        CaptionChild::Progress(builder.build())
    }
}
impl From<Quote> for CaptionChild {
    fn from(child: Quote) -> Self {
        CaptionChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for CaptionChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        CaptionChild::Quote(builder.build())
    }
}
impl From<Ruby> for CaptionChild {
    fn from(child: Ruby) -> Self {
        CaptionChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for CaptionChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        CaptionChild::Ruby(builder.build())
    }
}
impl From<Sample> for CaptionChild {
    fn from(child: Sample) -> Self {
        CaptionChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for CaptionChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        CaptionChild::Sample(builder.build())
    }
}
impl From<Script> for CaptionChild {
    fn from(child: Script) -> Self {
        CaptionChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for CaptionChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        CaptionChild::Script(builder.build())
    }
}
impl From<Search> for CaptionChild {
    fn from(child: Search) -> Self {
        CaptionChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for CaptionChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        CaptionChild::Search(builder.build())
    }
}
impl From<Section> for CaptionChild {
    fn from(child: Section) -> Self {
        CaptionChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for CaptionChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        CaptionChild::Section(builder.build())
    }
}
impl From<Select> for CaptionChild {
    fn from(child: Select) -> Self {
        CaptionChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for CaptionChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        CaptionChild::Select(builder.build())
    }
}
impl From<Slot> for CaptionChild {
    fn from(child: Slot) -> Self {
        CaptionChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for CaptionChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        CaptionChild::Slot(builder.build())
    }
}
impl From<Small> for CaptionChild {
    fn from(child: Small) -> Self {
        CaptionChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for CaptionChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        CaptionChild::Small(builder.build())
    }
}
impl From<Span> for CaptionChild {
    fn from(child: Span) -> Self {
        CaptionChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for CaptionChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        CaptionChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for CaptionChild {
    fn from(child: StrikeThrough) -> Self {
        CaptionChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for CaptionChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        CaptionChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for CaptionChild {
    fn from(child: Strong) -> Self {
        CaptionChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for CaptionChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        CaptionChild::Strong(builder.build())
    }
}
impl From<SubScript> for CaptionChild {
    fn from(child: SubScript) -> Self {
        CaptionChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for CaptionChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        CaptionChild::SubScript(builder.build())
    }
}
impl From<SupScript> for CaptionChild {
    fn from(child: SupScript) -> Self {
        CaptionChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for CaptionChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        CaptionChild::SupScript(builder.build())
    }
}
impl From<Table> for CaptionChild {
    fn from(child: Table) -> Self {
        CaptionChild::Table(child)
    }
}
impl From<builders::TableBuilder> for CaptionChild {
    fn from(builder: builders::TableBuilder) -> Self {
        CaptionChild::Table(builder.build())
    }
}
impl From<Template> for CaptionChild {
    fn from(child: Template) -> Self {
        CaptionChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for CaptionChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        CaptionChild::Template(builder.build())
    }
}
impl From<TextArea> for CaptionChild {
    fn from(child: TextArea) -> Self {
        CaptionChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for CaptionChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        CaptionChild::TextArea(builder.build())
    }
}
impl From<Time> for CaptionChild {
    fn from(child: Time) -> Self {
        CaptionChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for CaptionChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        CaptionChild::Time(builder.build())
    }
}
impl From<Underline> for CaptionChild {
    fn from(child: Underline) -> Self {
        CaptionChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for CaptionChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        CaptionChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for CaptionChild {
    fn from(child: UnorderedList) -> Self {
        CaptionChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for CaptionChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        CaptionChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for CaptionChild {
    fn from(child: Variable) -> Self {
        CaptionChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for CaptionChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        CaptionChild::Variable(builder.build())
    }
}
impl From<Video> for CaptionChild {
    fn from(child: Video) -> Self {
        CaptionChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for CaptionChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        CaptionChild::Video(builder.build())
    }
}
impl From<WordBreak> for CaptionChild {
    fn from(child: WordBreak) -> Self {
        CaptionChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for CaptionChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        CaptionChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for CaptionChild {
    fn from(s: &'static str) -> Self {
        CaptionChild::Text(s.into())
    }
}
impl From<String> for CaptionChild {
    fn from(s: String) -> Self {
        CaptionChild::Text(s.into())
    }
}
impl From<CowStr> for CaptionChild {
    fn from(s: CowStr) -> Self {
        CaptionChild::Text(s)
    }
}
impl std::fmt::Debug for CaptionChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CaptionChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Address(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Article(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Button(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Code(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Data(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Details(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Division(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Form(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Header(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Image(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Input(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Label(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Link(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Main(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Map(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Object(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Output(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Script(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Search(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Section(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Select(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Small(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Span(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Table(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Template(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Time(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Video(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            CaptionChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for CaptionChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CaptionChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Address(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Article(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Aside(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Audio(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            CaptionChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Bold(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Button(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Cite(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Code(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Custom(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Data(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::DataList(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Definition(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Details(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Division(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Embed(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Figure(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Footer(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Form(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Header(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Image(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Input(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Italic(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Label(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Link(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Main(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Map(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Mark(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Menu(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Meter(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Object(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Output(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Picture(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Progress(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Quote(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Sample(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Script(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Search(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Section(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Select(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Slot(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Small(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Span(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Strong(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Table(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Template(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Time(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Underline(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Variable(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Video(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            CaptionChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
