// 🤖 This file is generated!

use crate::*;
/// The `<footer>` element's children.
#[derive(Clone)]
pub enum FooterChild {
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
impl From<Abbreviation> for FooterChild {
    fn from(child: Abbreviation) -> Self {
        FooterChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for FooterChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        FooterChild::Abbreviation(builder.build())
    }
}
impl From<Address> for FooterChild {
    fn from(child: Address) -> Self {
        FooterChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for FooterChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        FooterChild::Address(builder.build())
    }
}
impl From<Anchor> for FooterChild {
    fn from(child: Anchor) -> Self {
        FooterChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for FooterChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        FooterChild::Anchor(builder.build())
    }
}
impl From<Article> for FooterChild {
    fn from(child: Article) -> Self {
        FooterChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for FooterChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        FooterChild::Article(builder.build())
    }
}
impl From<Aside> for FooterChild {
    fn from(child: Aside) -> Self {
        FooterChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for FooterChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        FooterChild::Aside(builder.build())
    }
}
impl From<Audio> for FooterChild {
    fn from(child: Audio) -> Self {
        FooterChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for FooterChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        FooterChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for FooterChild {
    fn from(child: BidirectionalIsolate) -> Self {
        FooterChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for FooterChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        FooterChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for FooterChild {
    fn from(child: BidirectionalOverride) -> Self {
        FooterChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for FooterChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        FooterChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for FooterChild {
    fn from(child: BlockQuote) -> Self {
        FooterChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for FooterChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        FooterChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for FooterChild {
    fn from(child: Bold) -> Self {
        FooterChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for FooterChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        FooterChild::Bold(builder.build())
    }
}
impl From<Button> for FooterChild {
    fn from(child: Button) -> Self {
        FooterChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for FooterChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        FooterChild::Button(builder.build())
    }
}
impl From<Canvas> for FooterChild {
    fn from(child: Canvas) -> Self {
        FooterChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for FooterChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        FooterChild::Canvas(builder.build())
    }
}
impl From<Cite> for FooterChild {
    fn from(child: Cite) -> Self {
        FooterChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for FooterChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        FooterChild::Cite(builder.build())
    }
}
impl From<Code> for FooterChild {
    fn from(child: Code) -> Self {
        FooterChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for FooterChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        FooterChild::Code(builder.build())
    }
}
impl From<Custom> for FooterChild {
    fn from(child: Custom) -> Self {
        FooterChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for FooterChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        FooterChild::Custom(builder.build())
    }
}
impl From<Data> for FooterChild {
    fn from(child: Data) -> Self {
        FooterChild::Data(child)
    }
}
impl From<builders::DataBuilder> for FooterChild {
    fn from(builder: builders::DataBuilder) -> Self {
        FooterChild::Data(builder.build())
    }
}
impl From<DataList> for FooterChild {
    fn from(child: DataList) -> Self {
        FooterChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for FooterChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        FooterChild::DataList(builder.build())
    }
}
impl From<Definition> for FooterChild {
    fn from(child: Definition) -> Self {
        FooterChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for FooterChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        FooterChild::Definition(builder.build())
    }
}
impl From<Deleted> for FooterChild {
    fn from(child: Deleted) -> Self {
        FooterChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for FooterChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        FooterChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for FooterChild {
    fn from(child: DescriptionList) -> Self {
        FooterChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for FooterChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        FooterChild::DescriptionList(builder.build())
    }
}
impl From<Details> for FooterChild {
    fn from(child: Details) -> Self {
        FooterChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for FooterChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        FooterChild::Details(builder.build())
    }
}
impl From<Dialog> for FooterChild {
    fn from(child: Dialog) -> Self {
        FooterChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for FooterChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        FooterChild::Dialog(builder.build())
    }
}
impl From<Division> for FooterChild {
    fn from(child: Division) -> Self {
        FooterChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for FooterChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        FooterChild::Division(builder.build())
    }
}
impl From<Embed> for FooterChild {
    fn from(child: Embed) -> Self {
        FooterChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for FooterChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        FooterChild::Embed(builder.build())
    }
}
impl From<Emphasis> for FooterChild {
    fn from(child: Emphasis) -> Self {
        FooterChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for FooterChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        FooterChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for FooterChild {
    fn from(child: FieldSet) -> Self {
        FooterChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for FooterChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        FooterChild::FieldSet(builder.build())
    }
}
impl From<Figure> for FooterChild {
    fn from(child: Figure) -> Self {
        FooterChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for FooterChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        FooterChild::Figure(builder.build())
    }
}
impl From<Footer> for FooterChild {
    fn from(child: Footer) -> Self {
        FooterChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for FooterChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        FooterChild::Footer(builder.build())
    }
}
impl From<Form> for FooterChild {
    fn from(child: Form) -> Self {
        FooterChild::Form(child)
    }
}
impl From<builders::FormBuilder> for FooterChild {
    fn from(builder: builders::FormBuilder) -> Self {
        FooterChild::Form(builder.build())
    }
}
impl From<Header> for FooterChild {
    fn from(child: Header) -> Self {
        FooterChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for FooterChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        FooterChild::Header(builder.build())
    }
}
impl From<Heading1> for FooterChild {
    fn from(child: Heading1) -> Self {
        FooterChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for FooterChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        FooterChild::Heading1(builder.build())
    }
}
impl From<Heading2> for FooterChild {
    fn from(child: Heading2) -> Self {
        FooterChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for FooterChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        FooterChild::Heading2(builder.build())
    }
}
impl From<Heading3> for FooterChild {
    fn from(child: Heading3) -> Self {
        FooterChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for FooterChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        FooterChild::Heading3(builder.build())
    }
}
impl From<Heading4> for FooterChild {
    fn from(child: Heading4) -> Self {
        FooterChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for FooterChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        FooterChild::Heading4(builder.build())
    }
}
impl From<Heading5> for FooterChild {
    fn from(child: Heading5) -> Self {
        FooterChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for FooterChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        FooterChild::Heading5(builder.build())
    }
}
impl From<Heading6> for FooterChild {
    fn from(child: Heading6) -> Self {
        FooterChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for FooterChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        FooterChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for FooterChild {
    fn from(child: HeadingGroup) -> Self {
        FooterChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for FooterChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        FooterChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for FooterChild {
    fn from(child: HorizontalRule) -> Self {
        FooterChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for FooterChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        FooterChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for FooterChild {
    fn from(child: Image) -> Self {
        FooterChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for FooterChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        FooterChild::Image(builder.build())
    }
}
impl From<InlineFrame> for FooterChild {
    fn from(child: InlineFrame) -> Self {
        FooterChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for FooterChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        FooterChild::InlineFrame(builder.build())
    }
}
impl From<Input> for FooterChild {
    fn from(child: Input) -> Self {
        FooterChild::Input(child)
    }
}
impl From<builders::InputBuilder> for FooterChild {
    fn from(builder: builders::InputBuilder) -> Self {
        FooterChild::Input(builder.build())
    }
}
impl From<Inserted> for FooterChild {
    fn from(child: Inserted) -> Self {
        FooterChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for FooterChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        FooterChild::Inserted(builder.build())
    }
}
impl From<Italic> for FooterChild {
    fn from(child: Italic) -> Self {
        FooterChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for FooterChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        FooterChild::Italic(builder.build())
    }
}
impl From<Keyboard> for FooterChild {
    fn from(child: Keyboard) -> Self {
        FooterChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for FooterChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        FooterChild::Keyboard(builder.build())
    }
}
impl From<Label> for FooterChild {
    fn from(child: Label) -> Self {
        FooterChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for FooterChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        FooterChild::Label(builder.build())
    }
}
impl From<LineBreak> for FooterChild {
    fn from(child: LineBreak) -> Self {
        FooterChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for FooterChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        FooterChild::LineBreak(builder.build())
    }
}
impl From<Link> for FooterChild {
    fn from(child: Link) -> Self {
        FooterChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for FooterChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        FooterChild::Link(builder.build())
    }
}
impl From<Main> for FooterChild {
    fn from(child: Main) -> Self {
        FooterChild::Main(child)
    }
}
impl From<builders::MainBuilder> for FooterChild {
    fn from(builder: builders::MainBuilder) -> Self {
        FooterChild::Main(builder.build())
    }
}
impl From<Map> for FooterChild {
    fn from(child: Map) -> Self {
        FooterChild::Map(child)
    }
}
impl From<builders::MapBuilder> for FooterChild {
    fn from(builder: builders::MapBuilder) -> Self {
        FooterChild::Map(builder.build())
    }
}
impl From<MapArea> for FooterChild {
    fn from(child: MapArea) -> Self {
        FooterChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for FooterChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        FooterChild::MapArea(builder.build())
    }
}
impl From<Mark> for FooterChild {
    fn from(child: Mark) -> Self {
        FooterChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for FooterChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        FooterChild::Mark(builder.build())
    }
}
impl From<Menu> for FooterChild {
    fn from(child: Menu) -> Self {
        FooterChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for FooterChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        FooterChild::Menu(builder.build())
    }
}
impl From<Metadata> for FooterChild {
    fn from(child: Metadata) -> Self {
        FooterChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for FooterChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        FooterChild::Metadata(builder.build())
    }
}
impl From<Meter> for FooterChild {
    fn from(child: Meter) -> Self {
        FooterChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for FooterChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        FooterChild::Meter(builder.build())
    }
}
impl From<Navigation> for FooterChild {
    fn from(child: Navigation) -> Self {
        FooterChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for FooterChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        FooterChild::Navigation(builder.build())
    }
}
impl From<NoScript> for FooterChild {
    fn from(child: NoScript) -> Self {
        FooterChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for FooterChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        FooterChild::NoScript(builder.build())
    }
}
impl From<Object> for FooterChild {
    fn from(child: Object) -> Self {
        FooterChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for FooterChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        FooterChild::Object(builder.build())
    }
}
impl From<OrderedList> for FooterChild {
    fn from(child: OrderedList) -> Self {
        FooterChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for FooterChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        FooterChild::OrderedList(builder.build())
    }
}
impl From<Output> for FooterChild {
    fn from(child: Output) -> Self {
        FooterChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for FooterChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        FooterChild::Output(builder.build())
    }
}
impl From<Paragraph> for FooterChild {
    fn from(child: Paragraph) -> Self {
        FooterChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for FooterChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        FooterChild::Paragraph(builder.build())
    }
}
impl From<Picture> for FooterChild {
    fn from(child: Picture) -> Self {
        FooterChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for FooterChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        FooterChild::Picture(builder.build())
    }
}
impl From<Preformatted> for FooterChild {
    fn from(child: Preformatted) -> Self {
        FooterChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for FooterChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        FooterChild::Preformatted(builder.build())
    }
}
impl From<Progress> for FooterChild {
    fn from(child: Progress) -> Self {
        FooterChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for FooterChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        FooterChild::Progress(builder.build())
    }
}
impl From<Quote> for FooterChild {
    fn from(child: Quote) -> Self {
        FooterChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for FooterChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        FooterChild::Quote(builder.build())
    }
}
impl From<Ruby> for FooterChild {
    fn from(child: Ruby) -> Self {
        FooterChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for FooterChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        FooterChild::Ruby(builder.build())
    }
}
impl From<Sample> for FooterChild {
    fn from(child: Sample) -> Self {
        FooterChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for FooterChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        FooterChild::Sample(builder.build())
    }
}
impl From<Script> for FooterChild {
    fn from(child: Script) -> Self {
        FooterChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for FooterChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        FooterChild::Script(builder.build())
    }
}
impl From<Search> for FooterChild {
    fn from(child: Search) -> Self {
        FooterChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for FooterChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        FooterChild::Search(builder.build())
    }
}
impl From<Section> for FooterChild {
    fn from(child: Section) -> Self {
        FooterChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for FooterChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        FooterChild::Section(builder.build())
    }
}
impl From<Select> for FooterChild {
    fn from(child: Select) -> Self {
        FooterChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for FooterChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        FooterChild::Select(builder.build())
    }
}
impl From<Slot> for FooterChild {
    fn from(child: Slot) -> Self {
        FooterChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for FooterChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        FooterChild::Slot(builder.build())
    }
}
impl From<Small> for FooterChild {
    fn from(child: Small) -> Self {
        FooterChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for FooterChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        FooterChild::Small(builder.build())
    }
}
impl From<Span> for FooterChild {
    fn from(child: Span) -> Self {
        FooterChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for FooterChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        FooterChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for FooterChild {
    fn from(child: StrikeThrough) -> Self {
        FooterChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for FooterChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        FooterChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for FooterChild {
    fn from(child: Strong) -> Self {
        FooterChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for FooterChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        FooterChild::Strong(builder.build())
    }
}
impl From<SubScript> for FooterChild {
    fn from(child: SubScript) -> Self {
        FooterChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for FooterChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        FooterChild::SubScript(builder.build())
    }
}
impl From<SupScript> for FooterChild {
    fn from(child: SupScript) -> Self {
        FooterChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for FooterChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        FooterChild::SupScript(builder.build())
    }
}
impl From<Table> for FooterChild {
    fn from(child: Table) -> Self {
        FooterChild::Table(child)
    }
}
impl From<builders::TableBuilder> for FooterChild {
    fn from(builder: builders::TableBuilder) -> Self {
        FooterChild::Table(builder.build())
    }
}
impl From<Template> for FooterChild {
    fn from(child: Template) -> Self {
        FooterChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for FooterChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        FooterChild::Template(builder.build())
    }
}
impl From<TextArea> for FooterChild {
    fn from(child: TextArea) -> Self {
        FooterChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for FooterChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        FooterChild::TextArea(builder.build())
    }
}
impl From<Time> for FooterChild {
    fn from(child: Time) -> Self {
        FooterChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for FooterChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        FooterChild::Time(builder.build())
    }
}
impl From<Underline> for FooterChild {
    fn from(child: Underline) -> Self {
        FooterChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for FooterChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        FooterChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for FooterChild {
    fn from(child: UnorderedList) -> Self {
        FooterChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for FooterChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        FooterChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for FooterChild {
    fn from(child: Variable) -> Self {
        FooterChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for FooterChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        FooterChild::Variable(builder.build())
    }
}
impl From<Video> for FooterChild {
    fn from(child: Video) -> Self {
        FooterChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for FooterChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        FooterChild::Video(builder.build())
    }
}
impl From<WordBreak> for FooterChild {
    fn from(child: WordBreak) -> Self {
        FooterChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for FooterChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        FooterChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for FooterChild {
    fn from(s: &'static str) -> Self {
        FooterChild::Text(s.into())
    }
}
impl From<String> for FooterChild {
    fn from(s: String) -> Self {
        FooterChild::Text(s.into())
    }
}
impl From<CowStr> for FooterChild {
    fn from(s: CowStr) -> Self {
        FooterChild::Text(s)
    }
}
impl std::fmt::Debug for FooterChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FooterChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Address(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Article(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Button(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Code(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Data(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Details(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Division(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Form(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Header(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Image(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Input(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Label(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Link(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Main(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Map(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Object(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Output(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Script(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Search(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Section(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Select(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Small(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Span(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Table(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Template(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Time(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Video(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            FooterChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for FooterChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FooterChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Address(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Article(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Aside(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Audio(child) => std::fmt::Display::fmt(child, f),
            FooterChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            FooterChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            FooterChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Bold(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Button(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Cite(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Code(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Custom(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Data(child) => std::fmt::Display::fmt(child, f),
            FooterChild::DataList(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Definition(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            FooterChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Details(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Division(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Embed(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            FooterChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Figure(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Footer(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Form(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Header(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            FooterChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            FooterChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Image(child) => std::fmt::Display::fmt(child, f),
            FooterChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Input(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Italic(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Label(child) => std::fmt::Display::fmt(child, f),
            FooterChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Link(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Main(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Map(child) => std::fmt::Display::fmt(child, f),
            FooterChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Mark(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Menu(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Meter(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            FooterChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Object(child) => std::fmt::Display::fmt(child, f),
            FooterChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Output(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Picture(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Progress(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Quote(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Sample(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Script(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Search(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Section(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Select(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Slot(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Small(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Span(child) => std::fmt::Display::fmt(child, f),
            FooterChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Strong(child) => std::fmt::Display::fmt(child, f),
            FooterChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            FooterChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Table(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Template(child) => std::fmt::Display::fmt(child, f),
            FooterChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Time(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Underline(child) => std::fmt::Display::fmt(child, f),
            FooterChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Variable(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Video(child) => std::fmt::Display::fmt(child, f),
            FooterChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            FooterChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
