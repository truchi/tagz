// 🤖 This file is generated!

use crate::*;
/// The `<nav>` element's children.
#[derive(Clone)]
pub enum NavigationChild {
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
impl From<Abbreviation> for NavigationChild {
    fn from(child: Abbreviation) -> Self {
        NavigationChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for NavigationChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        NavigationChild::Abbreviation(builder.build())
    }
}
impl From<Address> for NavigationChild {
    fn from(child: Address) -> Self {
        NavigationChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for NavigationChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        NavigationChild::Address(builder.build())
    }
}
impl From<Anchor> for NavigationChild {
    fn from(child: Anchor) -> Self {
        NavigationChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for NavigationChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        NavigationChild::Anchor(builder.build())
    }
}
impl From<Article> for NavigationChild {
    fn from(child: Article) -> Self {
        NavigationChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for NavigationChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        NavigationChild::Article(builder.build())
    }
}
impl From<Aside> for NavigationChild {
    fn from(child: Aside) -> Self {
        NavigationChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for NavigationChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        NavigationChild::Aside(builder.build())
    }
}
impl From<Audio> for NavigationChild {
    fn from(child: Audio) -> Self {
        NavigationChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for NavigationChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        NavigationChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for NavigationChild {
    fn from(child: BidirectionalIsolate) -> Self {
        NavigationChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for NavigationChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        NavigationChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for NavigationChild {
    fn from(child: BidirectionalOverride) -> Self {
        NavigationChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for NavigationChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        NavigationChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for NavigationChild {
    fn from(child: BlockQuote) -> Self {
        NavigationChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for NavigationChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        NavigationChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for NavigationChild {
    fn from(child: Bold) -> Self {
        NavigationChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for NavigationChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        NavigationChild::Bold(builder.build())
    }
}
impl From<Button> for NavigationChild {
    fn from(child: Button) -> Self {
        NavigationChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for NavigationChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        NavigationChild::Button(builder.build())
    }
}
impl From<Canvas> for NavigationChild {
    fn from(child: Canvas) -> Self {
        NavigationChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for NavigationChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        NavigationChild::Canvas(builder.build())
    }
}
impl From<Cite> for NavigationChild {
    fn from(child: Cite) -> Self {
        NavigationChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for NavigationChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        NavigationChild::Cite(builder.build())
    }
}
impl From<Code> for NavigationChild {
    fn from(child: Code) -> Self {
        NavigationChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for NavigationChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        NavigationChild::Code(builder.build())
    }
}
impl From<Custom> for NavigationChild {
    fn from(child: Custom) -> Self {
        NavigationChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for NavigationChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        NavigationChild::Custom(builder.build())
    }
}
impl From<Data> for NavigationChild {
    fn from(child: Data) -> Self {
        NavigationChild::Data(child)
    }
}
impl From<builders::DataBuilder> for NavigationChild {
    fn from(builder: builders::DataBuilder) -> Self {
        NavigationChild::Data(builder.build())
    }
}
impl From<DataList> for NavigationChild {
    fn from(child: DataList) -> Self {
        NavigationChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for NavigationChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        NavigationChild::DataList(builder.build())
    }
}
impl From<Definition> for NavigationChild {
    fn from(child: Definition) -> Self {
        NavigationChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for NavigationChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        NavigationChild::Definition(builder.build())
    }
}
impl From<Deleted> for NavigationChild {
    fn from(child: Deleted) -> Self {
        NavigationChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for NavigationChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        NavigationChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for NavigationChild {
    fn from(child: DescriptionList) -> Self {
        NavigationChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for NavigationChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        NavigationChild::DescriptionList(builder.build())
    }
}
impl From<Details> for NavigationChild {
    fn from(child: Details) -> Self {
        NavigationChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for NavigationChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        NavigationChild::Details(builder.build())
    }
}
impl From<Dialog> for NavigationChild {
    fn from(child: Dialog) -> Self {
        NavigationChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for NavigationChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        NavigationChild::Dialog(builder.build())
    }
}
impl From<Division> for NavigationChild {
    fn from(child: Division) -> Self {
        NavigationChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for NavigationChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        NavigationChild::Division(builder.build())
    }
}
impl From<Embed> for NavigationChild {
    fn from(child: Embed) -> Self {
        NavigationChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for NavigationChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        NavigationChild::Embed(builder.build())
    }
}
impl From<Emphasis> for NavigationChild {
    fn from(child: Emphasis) -> Self {
        NavigationChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for NavigationChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        NavigationChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for NavigationChild {
    fn from(child: FieldSet) -> Self {
        NavigationChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for NavigationChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        NavigationChild::FieldSet(builder.build())
    }
}
impl From<Figure> for NavigationChild {
    fn from(child: Figure) -> Self {
        NavigationChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for NavigationChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        NavigationChild::Figure(builder.build())
    }
}
impl From<Footer> for NavigationChild {
    fn from(child: Footer) -> Self {
        NavigationChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for NavigationChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        NavigationChild::Footer(builder.build())
    }
}
impl From<Form> for NavigationChild {
    fn from(child: Form) -> Self {
        NavigationChild::Form(child)
    }
}
impl From<builders::FormBuilder> for NavigationChild {
    fn from(builder: builders::FormBuilder) -> Self {
        NavigationChild::Form(builder.build())
    }
}
impl From<Header> for NavigationChild {
    fn from(child: Header) -> Self {
        NavigationChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for NavigationChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        NavigationChild::Header(builder.build())
    }
}
impl From<Heading1> for NavigationChild {
    fn from(child: Heading1) -> Self {
        NavigationChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for NavigationChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        NavigationChild::Heading1(builder.build())
    }
}
impl From<Heading2> for NavigationChild {
    fn from(child: Heading2) -> Self {
        NavigationChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for NavigationChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        NavigationChild::Heading2(builder.build())
    }
}
impl From<Heading3> for NavigationChild {
    fn from(child: Heading3) -> Self {
        NavigationChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for NavigationChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        NavigationChild::Heading3(builder.build())
    }
}
impl From<Heading4> for NavigationChild {
    fn from(child: Heading4) -> Self {
        NavigationChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for NavigationChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        NavigationChild::Heading4(builder.build())
    }
}
impl From<Heading5> for NavigationChild {
    fn from(child: Heading5) -> Self {
        NavigationChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for NavigationChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        NavigationChild::Heading5(builder.build())
    }
}
impl From<Heading6> for NavigationChild {
    fn from(child: Heading6) -> Self {
        NavigationChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for NavigationChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        NavigationChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for NavigationChild {
    fn from(child: HeadingGroup) -> Self {
        NavigationChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for NavigationChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        NavigationChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for NavigationChild {
    fn from(child: HorizontalRule) -> Self {
        NavigationChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for NavigationChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        NavigationChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for NavigationChild {
    fn from(child: Image) -> Self {
        NavigationChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for NavigationChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        NavigationChild::Image(builder.build())
    }
}
impl From<InlineFrame> for NavigationChild {
    fn from(child: InlineFrame) -> Self {
        NavigationChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for NavigationChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        NavigationChild::InlineFrame(builder.build())
    }
}
impl From<Input> for NavigationChild {
    fn from(child: Input) -> Self {
        NavigationChild::Input(child)
    }
}
impl From<builders::InputBuilder> for NavigationChild {
    fn from(builder: builders::InputBuilder) -> Self {
        NavigationChild::Input(builder.build())
    }
}
impl From<Inserted> for NavigationChild {
    fn from(child: Inserted) -> Self {
        NavigationChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for NavigationChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        NavigationChild::Inserted(builder.build())
    }
}
impl From<Italic> for NavigationChild {
    fn from(child: Italic) -> Self {
        NavigationChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for NavigationChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        NavigationChild::Italic(builder.build())
    }
}
impl From<Keyboard> for NavigationChild {
    fn from(child: Keyboard) -> Self {
        NavigationChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for NavigationChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        NavigationChild::Keyboard(builder.build())
    }
}
impl From<Label> for NavigationChild {
    fn from(child: Label) -> Self {
        NavigationChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for NavigationChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        NavigationChild::Label(builder.build())
    }
}
impl From<LineBreak> for NavigationChild {
    fn from(child: LineBreak) -> Self {
        NavigationChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for NavigationChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        NavigationChild::LineBreak(builder.build())
    }
}
impl From<Link> for NavigationChild {
    fn from(child: Link) -> Self {
        NavigationChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for NavigationChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        NavigationChild::Link(builder.build())
    }
}
impl From<Main> for NavigationChild {
    fn from(child: Main) -> Self {
        NavigationChild::Main(child)
    }
}
impl From<builders::MainBuilder> for NavigationChild {
    fn from(builder: builders::MainBuilder) -> Self {
        NavigationChild::Main(builder.build())
    }
}
impl From<Map> for NavigationChild {
    fn from(child: Map) -> Self {
        NavigationChild::Map(child)
    }
}
impl From<builders::MapBuilder> for NavigationChild {
    fn from(builder: builders::MapBuilder) -> Self {
        NavigationChild::Map(builder.build())
    }
}
impl From<MapArea> for NavigationChild {
    fn from(child: MapArea) -> Self {
        NavigationChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for NavigationChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        NavigationChild::MapArea(builder.build())
    }
}
impl From<Mark> for NavigationChild {
    fn from(child: Mark) -> Self {
        NavigationChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for NavigationChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        NavigationChild::Mark(builder.build())
    }
}
impl From<Menu> for NavigationChild {
    fn from(child: Menu) -> Self {
        NavigationChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for NavigationChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        NavigationChild::Menu(builder.build())
    }
}
impl From<Metadata> for NavigationChild {
    fn from(child: Metadata) -> Self {
        NavigationChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for NavigationChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        NavigationChild::Metadata(builder.build())
    }
}
impl From<Meter> for NavigationChild {
    fn from(child: Meter) -> Self {
        NavigationChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for NavigationChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        NavigationChild::Meter(builder.build())
    }
}
impl From<Navigation> for NavigationChild {
    fn from(child: Navigation) -> Self {
        NavigationChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for NavigationChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        NavigationChild::Navigation(builder.build())
    }
}
impl From<NoScript> for NavigationChild {
    fn from(child: NoScript) -> Self {
        NavigationChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for NavigationChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        NavigationChild::NoScript(builder.build())
    }
}
impl From<Object> for NavigationChild {
    fn from(child: Object) -> Self {
        NavigationChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for NavigationChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        NavigationChild::Object(builder.build())
    }
}
impl From<OrderedList> for NavigationChild {
    fn from(child: OrderedList) -> Self {
        NavigationChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for NavigationChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        NavigationChild::OrderedList(builder.build())
    }
}
impl From<Output> for NavigationChild {
    fn from(child: Output) -> Self {
        NavigationChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for NavigationChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        NavigationChild::Output(builder.build())
    }
}
impl From<Paragraph> for NavigationChild {
    fn from(child: Paragraph) -> Self {
        NavigationChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for NavigationChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        NavigationChild::Paragraph(builder.build())
    }
}
impl From<Picture> for NavigationChild {
    fn from(child: Picture) -> Self {
        NavigationChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for NavigationChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        NavigationChild::Picture(builder.build())
    }
}
impl From<Preformatted> for NavigationChild {
    fn from(child: Preformatted) -> Self {
        NavigationChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for NavigationChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        NavigationChild::Preformatted(builder.build())
    }
}
impl From<Progress> for NavigationChild {
    fn from(child: Progress) -> Self {
        NavigationChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for NavigationChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        NavigationChild::Progress(builder.build())
    }
}
impl From<Quote> for NavigationChild {
    fn from(child: Quote) -> Self {
        NavigationChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for NavigationChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        NavigationChild::Quote(builder.build())
    }
}
impl From<Ruby> for NavigationChild {
    fn from(child: Ruby) -> Self {
        NavigationChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for NavigationChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        NavigationChild::Ruby(builder.build())
    }
}
impl From<Sample> for NavigationChild {
    fn from(child: Sample) -> Self {
        NavigationChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for NavigationChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        NavigationChild::Sample(builder.build())
    }
}
impl From<Script> for NavigationChild {
    fn from(child: Script) -> Self {
        NavigationChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for NavigationChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        NavigationChild::Script(builder.build())
    }
}
impl From<Search> for NavigationChild {
    fn from(child: Search) -> Self {
        NavigationChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for NavigationChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        NavigationChild::Search(builder.build())
    }
}
impl From<Section> for NavigationChild {
    fn from(child: Section) -> Self {
        NavigationChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for NavigationChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        NavigationChild::Section(builder.build())
    }
}
impl From<Select> for NavigationChild {
    fn from(child: Select) -> Self {
        NavigationChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for NavigationChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        NavigationChild::Select(builder.build())
    }
}
impl From<Slot> for NavigationChild {
    fn from(child: Slot) -> Self {
        NavigationChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for NavigationChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        NavigationChild::Slot(builder.build())
    }
}
impl From<Small> for NavigationChild {
    fn from(child: Small) -> Self {
        NavigationChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for NavigationChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        NavigationChild::Small(builder.build())
    }
}
impl From<Span> for NavigationChild {
    fn from(child: Span) -> Self {
        NavigationChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for NavigationChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        NavigationChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for NavigationChild {
    fn from(child: StrikeThrough) -> Self {
        NavigationChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for NavigationChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        NavigationChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for NavigationChild {
    fn from(child: Strong) -> Self {
        NavigationChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for NavigationChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        NavigationChild::Strong(builder.build())
    }
}
impl From<SubScript> for NavigationChild {
    fn from(child: SubScript) -> Self {
        NavigationChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for NavigationChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        NavigationChild::SubScript(builder.build())
    }
}
impl From<SupScript> for NavigationChild {
    fn from(child: SupScript) -> Self {
        NavigationChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for NavigationChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        NavigationChild::SupScript(builder.build())
    }
}
impl From<Table> for NavigationChild {
    fn from(child: Table) -> Self {
        NavigationChild::Table(child)
    }
}
impl From<builders::TableBuilder> for NavigationChild {
    fn from(builder: builders::TableBuilder) -> Self {
        NavigationChild::Table(builder.build())
    }
}
impl From<Template> for NavigationChild {
    fn from(child: Template) -> Self {
        NavigationChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for NavigationChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        NavigationChild::Template(builder.build())
    }
}
impl From<TextArea> for NavigationChild {
    fn from(child: TextArea) -> Self {
        NavigationChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for NavigationChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        NavigationChild::TextArea(builder.build())
    }
}
impl From<Time> for NavigationChild {
    fn from(child: Time) -> Self {
        NavigationChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for NavigationChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        NavigationChild::Time(builder.build())
    }
}
impl From<Underline> for NavigationChild {
    fn from(child: Underline) -> Self {
        NavigationChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for NavigationChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        NavigationChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for NavigationChild {
    fn from(child: UnorderedList) -> Self {
        NavigationChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for NavigationChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        NavigationChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for NavigationChild {
    fn from(child: Variable) -> Self {
        NavigationChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for NavigationChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        NavigationChild::Variable(builder.build())
    }
}
impl From<Video> for NavigationChild {
    fn from(child: Video) -> Self {
        NavigationChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for NavigationChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        NavigationChild::Video(builder.build())
    }
}
impl From<WordBreak> for NavigationChild {
    fn from(child: WordBreak) -> Self {
        NavigationChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for NavigationChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        NavigationChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for NavigationChild {
    fn from(s: &'static str) -> Self {
        NavigationChild::Text(s.into())
    }
}
impl From<String> for NavigationChild {
    fn from(s: String) -> Self {
        NavigationChild::Text(s.into())
    }
}
impl From<CowStr> for NavigationChild {
    fn from(s: CowStr) -> Self {
        NavigationChild::Text(s)
    }
}
impl std::fmt::Debug for NavigationChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NavigationChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Address(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Article(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::BidirectionalIsolate(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            NavigationChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            NavigationChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Button(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Code(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Data(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Details(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Division(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Form(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Header(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Image(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Input(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Label(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Link(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Main(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Map(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Object(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Output(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Script(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Search(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Section(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Select(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Small(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Span(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Table(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Template(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Time(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Video(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            NavigationChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for NavigationChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NavigationChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Address(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Article(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Aside(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Audio(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            NavigationChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            NavigationChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Bold(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Button(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Cite(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Code(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Custom(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Data(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::DataList(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Definition(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Details(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Division(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Embed(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Figure(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Footer(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Form(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Header(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Image(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Input(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Italic(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Label(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Link(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Main(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Map(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Mark(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Menu(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Meter(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Object(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Output(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Picture(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Progress(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Quote(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Sample(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Script(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Search(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Section(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Select(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Slot(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Small(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Span(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Strong(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Table(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Template(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Time(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Underline(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Variable(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Video(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            NavigationChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
