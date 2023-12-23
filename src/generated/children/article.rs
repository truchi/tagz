// 🤖 This file is generated!

use crate::*;
/// The `<article>` element's children.
#[derive(Clone)]
pub enum ArticleChild {
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
impl From<Abbreviation> for ArticleChild {
    fn from(child: Abbreviation) -> Self {
        ArticleChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for ArticleChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        ArticleChild::Abbreviation(builder.build())
    }
}
impl From<Address> for ArticleChild {
    fn from(child: Address) -> Self {
        ArticleChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for ArticleChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        ArticleChild::Address(builder.build())
    }
}
impl From<Anchor> for ArticleChild {
    fn from(child: Anchor) -> Self {
        ArticleChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for ArticleChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        ArticleChild::Anchor(builder.build())
    }
}
impl From<Article> for ArticleChild {
    fn from(child: Article) -> Self {
        ArticleChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for ArticleChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        ArticleChild::Article(builder.build())
    }
}
impl From<Aside> for ArticleChild {
    fn from(child: Aside) -> Self {
        ArticleChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for ArticleChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        ArticleChild::Aside(builder.build())
    }
}
impl From<Audio> for ArticleChild {
    fn from(child: Audio) -> Self {
        ArticleChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for ArticleChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        ArticleChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for ArticleChild {
    fn from(child: BidirectionalIsolate) -> Self {
        ArticleChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for ArticleChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        ArticleChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for ArticleChild {
    fn from(child: BidirectionalOverride) -> Self {
        ArticleChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for ArticleChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        ArticleChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for ArticleChild {
    fn from(child: BlockQuote) -> Self {
        ArticleChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for ArticleChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        ArticleChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for ArticleChild {
    fn from(child: Bold) -> Self {
        ArticleChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for ArticleChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        ArticleChild::Bold(builder.build())
    }
}
impl From<Button> for ArticleChild {
    fn from(child: Button) -> Self {
        ArticleChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for ArticleChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        ArticleChild::Button(builder.build())
    }
}
impl From<Canvas> for ArticleChild {
    fn from(child: Canvas) -> Self {
        ArticleChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for ArticleChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        ArticleChild::Canvas(builder.build())
    }
}
impl From<Cite> for ArticleChild {
    fn from(child: Cite) -> Self {
        ArticleChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for ArticleChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        ArticleChild::Cite(builder.build())
    }
}
impl From<Code> for ArticleChild {
    fn from(child: Code) -> Self {
        ArticleChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for ArticleChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        ArticleChild::Code(builder.build())
    }
}
impl From<Custom> for ArticleChild {
    fn from(child: Custom) -> Self {
        ArticleChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for ArticleChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        ArticleChild::Custom(builder.build())
    }
}
impl From<Data> for ArticleChild {
    fn from(child: Data) -> Self {
        ArticleChild::Data(child)
    }
}
impl From<builders::DataBuilder> for ArticleChild {
    fn from(builder: builders::DataBuilder) -> Self {
        ArticleChild::Data(builder.build())
    }
}
impl From<DataList> for ArticleChild {
    fn from(child: DataList) -> Self {
        ArticleChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for ArticleChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        ArticleChild::DataList(builder.build())
    }
}
impl From<Definition> for ArticleChild {
    fn from(child: Definition) -> Self {
        ArticleChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for ArticleChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        ArticleChild::Definition(builder.build())
    }
}
impl From<Deleted> for ArticleChild {
    fn from(child: Deleted) -> Self {
        ArticleChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for ArticleChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        ArticleChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for ArticleChild {
    fn from(child: DescriptionList) -> Self {
        ArticleChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for ArticleChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        ArticleChild::DescriptionList(builder.build())
    }
}
impl From<Details> for ArticleChild {
    fn from(child: Details) -> Self {
        ArticleChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for ArticleChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        ArticleChild::Details(builder.build())
    }
}
impl From<Dialog> for ArticleChild {
    fn from(child: Dialog) -> Self {
        ArticleChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for ArticleChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        ArticleChild::Dialog(builder.build())
    }
}
impl From<Division> for ArticleChild {
    fn from(child: Division) -> Self {
        ArticleChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for ArticleChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        ArticleChild::Division(builder.build())
    }
}
impl From<Embed> for ArticleChild {
    fn from(child: Embed) -> Self {
        ArticleChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for ArticleChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        ArticleChild::Embed(builder.build())
    }
}
impl From<Emphasis> for ArticleChild {
    fn from(child: Emphasis) -> Self {
        ArticleChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for ArticleChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        ArticleChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for ArticleChild {
    fn from(child: FieldSet) -> Self {
        ArticleChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for ArticleChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        ArticleChild::FieldSet(builder.build())
    }
}
impl From<Figure> for ArticleChild {
    fn from(child: Figure) -> Self {
        ArticleChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for ArticleChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        ArticleChild::Figure(builder.build())
    }
}
impl From<Footer> for ArticleChild {
    fn from(child: Footer) -> Self {
        ArticleChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for ArticleChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        ArticleChild::Footer(builder.build())
    }
}
impl From<Form> for ArticleChild {
    fn from(child: Form) -> Self {
        ArticleChild::Form(child)
    }
}
impl From<builders::FormBuilder> for ArticleChild {
    fn from(builder: builders::FormBuilder) -> Self {
        ArticleChild::Form(builder.build())
    }
}
impl From<Header> for ArticleChild {
    fn from(child: Header) -> Self {
        ArticleChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for ArticleChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        ArticleChild::Header(builder.build())
    }
}
impl From<Heading1> for ArticleChild {
    fn from(child: Heading1) -> Self {
        ArticleChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for ArticleChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        ArticleChild::Heading1(builder.build())
    }
}
impl From<Heading2> for ArticleChild {
    fn from(child: Heading2) -> Self {
        ArticleChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for ArticleChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        ArticleChild::Heading2(builder.build())
    }
}
impl From<Heading3> for ArticleChild {
    fn from(child: Heading3) -> Self {
        ArticleChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for ArticleChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        ArticleChild::Heading3(builder.build())
    }
}
impl From<Heading4> for ArticleChild {
    fn from(child: Heading4) -> Self {
        ArticleChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for ArticleChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        ArticleChild::Heading4(builder.build())
    }
}
impl From<Heading5> for ArticleChild {
    fn from(child: Heading5) -> Self {
        ArticleChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for ArticleChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        ArticleChild::Heading5(builder.build())
    }
}
impl From<Heading6> for ArticleChild {
    fn from(child: Heading6) -> Self {
        ArticleChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for ArticleChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        ArticleChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for ArticleChild {
    fn from(child: HeadingGroup) -> Self {
        ArticleChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for ArticleChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        ArticleChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for ArticleChild {
    fn from(child: HorizontalRule) -> Self {
        ArticleChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for ArticleChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        ArticleChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for ArticleChild {
    fn from(child: Image) -> Self {
        ArticleChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for ArticleChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        ArticleChild::Image(builder.build())
    }
}
impl From<InlineFrame> for ArticleChild {
    fn from(child: InlineFrame) -> Self {
        ArticleChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for ArticleChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        ArticleChild::InlineFrame(builder.build())
    }
}
impl From<Input> for ArticleChild {
    fn from(child: Input) -> Self {
        ArticleChild::Input(child)
    }
}
impl From<builders::InputBuilder> for ArticleChild {
    fn from(builder: builders::InputBuilder) -> Self {
        ArticleChild::Input(builder.build())
    }
}
impl From<Inserted> for ArticleChild {
    fn from(child: Inserted) -> Self {
        ArticleChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for ArticleChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        ArticleChild::Inserted(builder.build())
    }
}
impl From<Italic> for ArticleChild {
    fn from(child: Italic) -> Self {
        ArticleChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for ArticleChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        ArticleChild::Italic(builder.build())
    }
}
impl From<Keyboard> for ArticleChild {
    fn from(child: Keyboard) -> Self {
        ArticleChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for ArticleChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        ArticleChild::Keyboard(builder.build())
    }
}
impl From<Label> for ArticleChild {
    fn from(child: Label) -> Self {
        ArticleChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for ArticleChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        ArticleChild::Label(builder.build())
    }
}
impl From<LineBreak> for ArticleChild {
    fn from(child: LineBreak) -> Self {
        ArticleChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for ArticleChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        ArticleChild::LineBreak(builder.build())
    }
}
impl From<Link> for ArticleChild {
    fn from(child: Link) -> Self {
        ArticleChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for ArticleChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        ArticleChild::Link(builder.build())
    }
}
impl From<Main> for ArticleChild {
    fn from(child: Main) -> Self {
        ArticleChild::Main(child)
    }
}
impl From<builders::MainBuilder> for ArticleChild {
    fn from(builder: builders::MainBuilder) -> Self {
        ArticleChild::Main(builder.build())
    }
}
impl From<Map> for ArticleChild {
    fn from(child: Map) -> Self {
        ArticleChild::Map(child)
    }
}
impl From<builders::MapBuilder> for ArticleChild {
    fn from(builder: builders::MapBuilder) -> Self {
        ArticleChild::Map(builder.build())
    }
}
impl From<MapArea> for ArticleChild {
    fn from(child: MapArea) -> Self {
        ArticleChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for ArticleChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        ArticleChild::MapArea(builder.build())
    }
}
impl From<Mark> for ArticleChild {
    fn from(child: Mark) -> Self {
        ArticleChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for ArticleChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        ArticleChild::Mark(builder.build())
    }
}
impl From<Menu> for ArticleChild {
    fn from(child: Menu) -> Self {
        ArticleChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for ArticleChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        ArticleChild::Menu(builder.build())
    }
}
impl From<Metadata> for ArticleChild {
    fn from(child: Metadata) -> Self {
        ArticleChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for ArticleChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        ArticleChild::Metadata(builder.build())
    }
}
impl From<Meter> for ArticleChild {
    fn from(child: Meter) -> Self {
        ArticleChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for ArticleChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        ArticleChild::Meter(builder.build())
    }
}
impl From<Navigation> for ArticleChild {
    fn from(child: Navigation) -> Self {
        ArticleChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for ArticleChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        ArticleChild::Navigation(builder.build())
    }
}
impl From<NoScript> for ArticleChild {
    fn from(child: NoScript) -> Self {
        ArticleChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for ArticleChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        ArticleChild::NoScript(builder.build())
    }
}
impl From<Object> for ArticleChild {
    fn from(child: Object) -> Self {
        ArticleChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for ArticleChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        ArticleChild::Object(builder.build())
    }
}
impl From<OrderedList> for ArticleChild {
    fn from(child: OrderedList) -> Self {
        ArticleChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for ArticleChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        ArticleChild::OrderedList(builder.build())
    }
}
impl From<Output> for ArticleChild {
    fn from(child: Output) -> Self {
        ArticleChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for ArticleChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        ArticleChild::Output(builder.build())
    }
}
impl From<Paragraph> for ArticleChild {
    fn from(child: Paragraph) -> Self {
        ArticleChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for ArticleChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        ArticleChild::Paragraph(builder.build())
    }
}
impl From<Picture> for ArticleChild {
    fn from(child: Picture) -> Self {
        ArticleChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for ArticleChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        ArticleChild::Picture(builder.build())
    }
}
impl From<Preformatted> for ArticleChild {
    fn from(child: Preformatted) -> Self {
        ArticleChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for ArticleChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        ArticleChild::Preformatted(builder.build())
    }
}
impl From<Progress> for ArticleChild {
    fn from(child: Progress) -> Self {
        ArticleChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for ArticleChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        ArticleChild::Progress(builder.build())
    }
}
impl From<Quote> for ArticleChild {
    fn from(child: Quote) -> Self {
        ArticleChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for ArticleChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        ArticleChild::Quote(builder.build())
    }
}
impl From<Ruby> for ArticleChild {
    fn from(child: Ruby) -> Self {
        ArticleChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for ArticleChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        ArticleChild::Ruby(builder.build())
    }
}
impl From<Sample> for ArticleChild {
    fn from(child: Sample) -> Self {
        ArticleChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for ArticleChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        ArticleChild::Sample(builder.build())
    }
}
impl From<Script> for ArticleChild {
    fn from(child: Script) -> Self {
        ArticleChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for ArticleChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        ArticleChild::Script(builder.build())
    }
}
impl From<Search> for ArticleChild {
    fn from(child: Search) -> Self {
        ArticleChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for ArticleChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        ArticleChild::Search(builder.build())
    }
}
impl From<Section> for ArticleChild {
    fn from(child: Section) -> Self {
        ArticleChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for ArticleChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        ArticleChild::Section(builder.build())
    }
}
impl From<Select> for ArticleChild {
    fn from(child: Select) -> Self {
        ArticleChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for ArticleChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        ArticleChild::Select(builder.build())
    }
}
impl From<Slot> for ArticleChild {
    fn from(child: Slot) -> Self {
        ArticleChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for ArticleChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        ArticleChild::Slot(builder.build())
    }
}
impl From<Small> for ArticleChild {
    fn from(child: Small) -> Self {
        ArticleChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for ArticleChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        ArticleChild::Small(builder.build())
    }
}
impl From<Span> for ArticleChild {
    fn from(child: Span) -> Self {
        ArticleChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for ArticleChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        ArticleChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for ArticleChild {
    fn from(child: StrikeThrough) -> Self {
        ArticleChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for ArticleChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        ArticleChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for ArticleChild {
    fn from(child: Strong) -> Self {
        ArticleChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for ArticleChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        ArticleChild::Strong(builder.build())
    }
}
impl From<SubScript> for ArticleChild {
    fn from(child: SubScript) -> Self {
        ArticleChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for ArticleChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        ArticleChild::SubScript(builder.build())
    }
}
impl From<SupScript> for ArticleChild {
    fn from(child: SupScript) -> Self {
        ArticleChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for ArticleChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        ArticleChild::SupScript(builder.build())
    }
}
impl From<Table> for ArticleChild {
    fn from(child: Table) -> Self {
        ArticleChild::Table(child)
    }
}
impl From<builders::TableBuilder> for ArticleChild {
    fn from(builder: builders::TableBuilder) -> Self {
        ArticleChild::Table(builder.build())
    }
}
impl From<Template> for ArticleChild {
    fn from(child: Template) -> Self {
        ArticleChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for ArticleChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        ArticleChild::Template(builder.build())
    }
}
impl From<TextArea> for ArticleChild {
    fn from(child: TextArea) -> Self {
        ArticleChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for ArticleChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        ArticleChild::TextArea(builder.build())
    }
}
impl From<Time> for ArticleChild {
    fn from(child: Time) -> Self {
        ArticleChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for ArticleChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        ArticleChild::Time(builder.build())
    }
}
impl From<Underline> for ArticleChild {
    fn from(child: Underline) -> Self {
        ArticleChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for ArticleChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        ArticleChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for ArticleChild {
    fn from(child: UnorderedList) -> Self {
        ArticleChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for ArticleChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        ArticleChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for ArticleChild {
    fn from(child: Variable) -> Self {
        ArticleChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for ArticleChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        ArticleChild::Variable(builder.build())
    }
}
impl From<Video> for ArticleChild {
    fn from(child: Video) -> Self {
        ArticleChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for ArticleChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        ArticleChild::Video(builder.build())
    }
}
impl From<WordBreak> for ArticleChild {
    fn from(child: WordBreak) -> Self {
        ArticleChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for ArticleChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        ArticleChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for ArticleChild {
    fn from(s: &'static str) -> Self {
        ArticleChild::Text(s.into())
    }
}
impl From<String> for ArticleChild {
    fn from(s: String) -> Self {
        ArticleChild::Text(s.into())
    }
}
impl From<CowStr> for ArticleChild {
    fn from(s: CowStr) -> Self {
        ArticleChild::Text(s)
    }
}
impl std::fmt::Debug for ArticleChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArticleChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Address(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Article(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Button(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Code(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Data(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Details(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Division(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Form(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Header(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Image(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Input(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Label(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Link(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Main(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Map(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Object(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Output(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Script(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Search(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Section(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Select(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Small(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Span(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Table(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Template(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Time(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Video(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            ArticleChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for ArticleChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArticleChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Address(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Article(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Aside(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Audio(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            ArticleChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Bold(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Button(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Cite(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Code(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Custom(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Data(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::DataList(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Definition(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Details(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Division(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Embed(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Figure(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Footer(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Form(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Header(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Image(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Input(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Italic(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Label(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Link(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Main(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Map(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Mark(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Menu(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Meter(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Object(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Output(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Picture(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Progress(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Quote(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Sample(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Script(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Search(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Section(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Select(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Slot(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Small(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Span(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Strong(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Table(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Template(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Time(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Underline(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Variable(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Video(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            ArticleChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
