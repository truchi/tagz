// 🤖 This file is generated!

use crate::*;
/// The `<ins>` element's children.
#[derive(Clone)]
pub enum InsertedChild {
    Abbreviation(Abbreviation),
    Address(Address),
    Anchor(Anchor),
    Article(Article),
    Aside(Aside),
    Audio(Audio),
    Base(Base),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    BlockQuote(BlockQuote),
    Body(Body),
    Bold(Bold),
    Button(Button),
    Canvas(Canvas),
    Caption(Caption),
    Cite(Cite),
    Code(Code),
    Custom(Custom),
    Data(Data),
    DataList(DataList),
    Definition(Definition),
    Deleted(Deleted),
    DescriptionDetails(DescriptionDetails),
    DescriptionList(DescriptionList),
    DescriptionTerm(DescriptionTerm),
    Details(Details),
    Dialog(Dialog),
    Division(Division),
    Embed(Embed),
    Emphasis(Emphasis),
    FieldSet(FieldSet),
    FieldSetLegend(FieldSetLegend),
    Figure(Figure),
    FigureCaption(FigureCaption),
    Footer(Footer),
    Form(Form),
    Head(Head),
    Header(Header),
    Heading1(Heading1),
    Heading2(Heading2),
    Heading3(Heading3),
    Heading4(Heading4),
    Heading5(Heading5),
    Heading6(Heading6),
    HeadingGroup(HeadingGroup),
    HorizontalRule(HorizontalRule),
    Html(Html),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    ListItem(ListItem),
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
    Option(Option),
    OptionGroup(OptionGroup),
    OrderedList(OrderedList),
    Output(Output),
    Paragraph(Paragraph),
    Picture(Picture),
    Preformatted(Preformatted),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    RubyParenthesis(RubyParenthesis),
    RubyText(RubyText),
    Sample(Sample),
    Script(Script),
    Search(Search),
    Section(Section),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Source(Source),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    Style(Style),
    SubScript(SubScript),
    Summary(Summary),
    SupScript(SupScript),
    Table(Table),
    TableBody(TableBody),
    TableCell(TableCell),
    TableColumn(TableColumn),
    TableColumnGroup(TableColumnGroup),
    TableFoot(TableFoot),
    TableHead(TableHead),
    TableHeader(TableHeader),
    TableRow(TableRow),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Title(Title),
    Track(Track),
    Underline(Underline),
    UnorderedList(UnorderedList),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
}
impl From<Abbreviation> for InsertedChild {
    fn from(child: Abbreviation) -> Self {
        InsertedChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for InsertedChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        InsertedChild::Abbreviation(builder.build())
    }
}
impl From<Address> for InsertedChild {
    fn from(child: Address) -> Self {
        InsertedChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for InsertedChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        InsertedChild::Address(builder.build())
    }
}
impl From<Anchor> for InsertedChild {
    fn from(child: Anchor) -> Self {
        InsertedChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for InsertedChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        InsertedChild::Anchor(builder.build())
    }
}
impl From<Article> for InsertedChild {
    fn from(child: Article) -> Self {
        InsertedChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for InsertedChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        InsertedChild::Article(builder.build())
    }
}
impl From<Aside> for InsertedChild {
    fn from(child: Aside) -> Self {
        InsertedChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for InsertedChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        InsertedChild::Aside(builder.build())
    }
}
impl From<Audio> for InsertedChild {
    fn from(child: Audio) -> Self {
        InsertedChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for InsertedChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        InsertedChild::Audio(builder.build())
    }
}
impl From<Base> for InsertedChild {
    fn from(child: Base) -> Self {
        InsertedChild::Base(child)
    }
}
impl From<builders::BaseBuilder> for InsertedChild {
    fn from(builder: builders::BaseBuilder) -> Self {
        InsertedChild::Base(builder.build())
    }
}
impl From<BidirectionalIsolate> for InsertedChild {
    fn from(child: BidirectionalIsolate) -> Self {
        InsertedChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for InsertedChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        InsertedChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for InsertedChild {
    fn from(child: BidirectionalOverride) -> Self {
        InsertedChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for InsertedChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        InsertedChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for InsertedChild {
    fn from(child: BlockQuote) -> Self {
        InsertedChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for InsertedChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        InsertedChild::BlockQuote(builder.build())
    }
}
impl From<Body> for InsertedChild {
    fn from(child: Body) -> Self {
        InsertedChild::Body(child)
    }
}
impl From<builders::BodyBuilder> for InsertedChild {
    fn from(builder: builders::BodyBuilder) -> Self {
        InsertedChild::Body(builder.build())
    }
}
impl From<Bold> for InsertedChild {
    fn from(child: Bold) -> Self {
        InsertedChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for InsertedChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        InsertedChild::Bold(builder.build())
    }
}
impl From<Button> for InsertedChild {
    fn from(child: Button) -> Self {
        InsertedChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for InsertedChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        InsertedChild::Button(builder.build())
    }
}
impl From<Canvas> for InsertedChild {
    fn from(child: Canvas) -> Self {
        InsertedChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for InsertedChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        InsertedChild::Canvas(builder.build())
    }
}
impl From<Caption> for InsertedChild {
    fn from(child: Caption) -> Self {
        InsertedChild::Caption(child)
    }
}
impl From<builders::CaptionBuilder> for InsertedChild {
    fn from(builder: builders::CaptionBuilder) -> Self {
        InsertedChild::Caption(builder.build())
    }
}
impl From<Cite> for InsertedChild {
    fn from(child: Cite) -> Self {
        InsertedChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for InsertedChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        InsertedChild::Cite(builder.build())
    }
}
impl From<Code> for InsertedChild {
    fn from(child: Code) -> Self {
        InsertedChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for InsertedChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        InsertedChild::Code(builder.build())
    }
}
impl From<Custom> for InsertedChild {
    fn from(child: Custom) -> Self {
        InsertedChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for InsertedChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        InsertedChild::Custom(builder.build())
    }
}
impl From<Data> for InsertedChild {
    fn from(child: Data) -> Self {
        InsertedChild::Data(child)
    }
}
impl From<builders::DataBuilder> for InsertedChild {
    fn from(builder: builders::DataBuilder) -> Self {
        InsertedChild::Data(builder.build())
    }
}
impl From<DataList> for InsertedChild {
    fn from(child: DataList) -> Self {
        InsertedChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for InsertedChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        InsertedChild::DataList(builder.build())
    }
}
impl From<Definition> for InsertedChild {
    fn from(child: Definition) -> Self {
        InsertedChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for InsertedChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        InsertedChild::Definition(builder.build())
    }
}
impl From<Deleted> for InsertedChild {
    fn from(child: Deleted) -> Self {
        InsertedChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for InsertedChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        InsertedChild::Deleted(builder.build())
    }
}
impl From<DescriptionDetails> for InsertedChild {
    fn from(child: DescriptionDetails) -> Self {
        InsertedChild::DescriptionDetails(child)
    }
}
impl From<builders::DescriptionDetailsBuilder> for InsertedChild {
    fn from(builder: builders::DescriptionDetailsBuilder) -> Self {
        InsertedChild::DescriptionDetails(builder.build())
    }
}
impl From<DescriptionList> for InsertedChild {
    fn from(child: DescriptionList) -> Self {
        InsertedChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for InsertedChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        InsertedChild::DescriptionList(builder.build())
    }
}
impl From<DescriptionTerm> for InsertedChild {
    fn from(child: DescriptionTerm) -> Self {
        InsertedChild::DescriptionTerm(child)
    }
}
impl From<builders::DescriptionTermBuilder> for InsertedChild {
    fn from(builder: builders::DescriptionTermBuilder) -> Self {
        InsertedChild::DescriptionTerm(builder.build())
    }
}
impl From<Details> for InsertedChild {
    fn from(child: Details) -> Self {
        InsertedChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for InsertedChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        InsertedChild::Details(builder.build())
    }
}
impl From<Dialog> for InsertedChild {
    fn from(child: Dialog) -> Self {
        InsertedChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for InsertedChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        InsertedChild::Dialog(builder.build())
    }
}
impl From<Division> for InsertedChild {
    fn from(child: Division) -> Self {
        InsertedChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for InsertedChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        InsertedChild::Division(builder.build())
    }
}
impl From<Embed> for InsertedChild {
    fn from(child: Embed) -> Self {
        InsertedChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for InsertedChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        InsertedChild::Embed(builder.build())
    }
}
impl From<Emphasis> for InsertedChild {
    fn from(child: Emphasis) -> Self {
        InsertedChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for InsertedChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        InsertedChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for InsertedChild {
    fn from(child: FieldSet) -> Self {
        InsertedChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for InsertedChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        InsertedChild::FieldSet(builder.build())
    }
}
impl From<FieldSetLegend> for InsertedChild {
    fn from(child: FieldSetLegend) -> Self {
        InsertedChild::FieldSetLegend(child)
    }
}
impl From<builders::FieldSetLegendBuilder> for InsertedChild {
    fn from(builder: builders::FieldSetLegendBuilder) -> Self {
        InsertedChild::FieldSetLegend(builder.build())
    }
}
impl From<Figure> for InsertedChild {
    fn from(child: Figure) -> Self {
        InsertedChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for InsertedChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        InsertedChild::Figure(builder.build())
    }
}
impl From<FigureCaption> for InsertedChild {
    fn from(child: FigureCaption) -> Self {
        InsertedChild::FigureCaption(child)
    }
}
impl From<builders::FigureCaptionBuilder> for InsertedChild {
    fn from(builder: builders::FigureCaptionBuilder) -> Self {
        InsertedChild::FigureCaption(builder.build())
    }
}
impl From<Footer> for InsertedChild {
    fn from(child: Footer) -> Self {
        InsertedChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for InsertedChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        InsertedChild::Footer(builder.build())
    }
}
impl From<Form> for InsertedChild {
    fn from(child: Form) -> Self {
        InsertedChild::Form(child)
    }
}
impl From<builders::FormBuilder> for InsertedChild {
    fn from(builder: builders::FormBuilder) -> Self {
        InsertedChild::Form(builder.build())
    }
}
impl From<Head> for InsertedChild {
    fn from(child: Head) -> Self {
        InsertedChild::Head(child)
    }
}
impl From<builders::HeadBuilder> for InsertedChild {
    fn from(builder: builders::HeadBuilder) -> Self {
        InsertedChild::Head(builder.build())
    }
}
impl From<Header> for InsertedChild {
    fn from(child: Header) -> Self {
        InsertedChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for InsertedChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        InsertedChild::Header(builder.build())
    }
}
impl From<Heading1> for InsertedChild {
    fn from(child: Heading1) -> Self {
        InsertedChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for InsertedChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        InsertedChild::Heading1(builder.build())
    }
}
impl From<Heading2> for InsertedChild {
    fn from(child: Heading2) -> Self {
        InsertedChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for InsertedChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        InsertedChild::Heading2(builder.build())
    }
}
impl From<Heading3> for InsertedChild {
    fn from(child: Heading3) -> Self {
        InsertedChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for InsertedChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        InsertedChild::Heading3(builder.build())
    }
}
impl From<Heading4> for InsertedChild {
    fn from(child: Heading4) -> Self {
        InsertedChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for InsertedChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        InsertedChild::Heading4(builder.build())
    }
}
impl From<Heading5> for InsertedChild {
    fn from(child: Heading5) -> Self {
        InsertedChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for InsertedChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        InsertedChild::Heading5(builder.build())
    }
}
impl From<Heading6> for InsertedChild {
    fn from(child: Heading6) -> Self {
        InsertedChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for InsertedChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        InsertedChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for InsertedChild {
    fn from(child: HeadingGroup) -> Self {
        InsertedChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for InsertedChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        InsertedChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for InsertedChild {
    fn from(child: HorizontalRule) -> Self {
        InsertedChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for InsertedChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        InsertedChild::HorizontalRule(builder.build())
    }
}
impl From<Html> for InsertedChild {
    fn from(child: Html) -> Self {
        InsertedChild::Html(child)
    }
}
impl From<builders::HtmlBuilder> for InsertedChild {
    fn from(builder: builders::HtmlBuilder) -> Self {
        InsertedChild::Html(builder.build())
    }
}
impl From<Image> for InsertedChild {
    fn from(child: Image) -> Self {
        InsertedChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for InsertedChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        InsertedChild::Image(builder.build())
    }
}
impl From<InlineFrame> for InsertedChild {
    fn from(child: InlineFrame) -> Self {
        InsertedChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for InsertedChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        InsertedChild::InlineFrame(builder.build())
    }
}
impl From<Input> for InsertedChild {
    fn from(child: Input) -> Self {
        InsertedChild::Input(child)
    }
}
impl From<builders::InputBuilder> for InsertedChild {
    fn from(builder: builders::InputBuilder) -> Self {
        InsertedChild::Input(builder.build())
    }
}
impl From<Inserted> for InsertedChild {
    fn from(child: Inserted) -> Self {
        InsertedChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for InsertedChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        InsertedChild::Inserted(builder.build())
    }
}
impl From<Italic> for InsertedChild {
    fn from(child: Italic) -> Self {
        InsertedChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for InsertedChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        InsertedChild::Italic(builder.build())
    }
}
impl From<Keyboard> for InsertedChild {
    fn from(child: Keyboard) -> Self {
        InsertedChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for InsertedChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        InsertedChild::Keyboard(builder.build())
    }
}
impl From<Label> for InsertedChild {
    fn from(child: Label) -> Self {
        InsertedChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for InsertedChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        InsertedChild::Label(builder.build())
    }
}
impl From<LineBreak> for InsertedChild {
    fn from(child: LineBreak) -> Self {
        InsertedChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for InsertedChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        InsertedChild::LineBreak(builder.build())
    }
}
impl From<Link> for InsertedChild {
    fn from(child: Link) -> Self {
        InsertedChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for InsertedChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        InsertedChild::Link(builder.build())
    }
}
impl From<ListItem> for InsertedChild {
    fn from(child: ListItem) -> Self {
        InsertedChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for InsertedChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        InsertedChild::ListItem(builder.build())
    }
}
impl From<Main> for InsertedChild {
    fn from(child: Main) -> Self {
        InsertedChild::Main(child)
    }
}
impl From<builders::MainBuilder> for InsertedChild {
    fn from(builder: builders::MainBuilder) -> Self {
        InsertedChild::Main(builder.build())
    }
}
impl From<Map> for InsertedChild {
    fn from(child: Map) -> Self {
        InsertedChild::Map(child)
    }
}
impl From<builders::MapBuilder> for InsertedChild {
    fn from(builder: builders::MapBuilder) -> Self {
        InsertedChild::Map(builder.build())
    }
}
impl From<MapArea> for InsertedChild {
    fn from(child: MapArea) -> Self {
        InsertedChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for InsertedChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        InsertedChild::MapArea(builder.build())
    }
}
impl From<Mark> for InsertedChild {
    fn from(child: Mark) -> Self {
        InsertedChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for InsertedChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        InsertedChild::Mark(builder.build())
    }
}
impl From<Menu> for InsertedChild {
    fn from(child: Menu) -> Self {
        InsertedChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for InsertedChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        InsertedChild::Menu(builder.build())
    }
}
impl From<Metadata> for InsertedChild {
    fn from(child: Metadata) -> Self {
        InsertedChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for InsertedChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        InsertedChild::Metadata(builder.build())
    }
}
impl From<Meter> for InsertedChild {
    fn from(child: Meter) -> Self {
        InsertedChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for InsertedChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        InsertedChild::Meter(builder.build())
    }
}
impl From<Navigation> for InsertedChild {
    fn from(child: Navigation) -> Self {
        InsertedChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for InsertedChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        InsertedChild::Navigation(builder.build())
    }
}
impl From<NoScript> for InsertedChild {
    fn from(child: NoScript) -> Self {
        InsertedChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for InsertedChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        InsertedChild::NoScript(builder.build())
    }
}
impl From<Object> for InsertedChild {
    fn from(child: Object) -> Self {
        InsertedChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for InsertedChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        InsertedChild::Object(builder.build())
    }
}
impl From<Option> for InsertedChild {
    fn from(child: Option) -> Self {
        InsertedChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for InsertedChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        InsertedChild::Option(builder.build())
    }
}
impl From<OptionGroup> for InsertedChild {
    fn from(child: OptionGroup) -> Self {
        InsertedChild::OptionGroup(child)
    }
}
impl From<builders::OptionGroupBuilder> for InsertedChild {
    fn from(builder: builders::OptionGroupBuilder) -> Self {
        InsertedChild::OptionGroup(builder.build())
    }
}
impl From<OrderedList> for InsertedChild {
    fn from(child: OrderedList) -> Self {
        InsertedChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for InsertedChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        InsertedChild::OrderedList(builder.build())
    }
}
impl From<Output> for InsertedChild {
    fn from(child: Output) -> Self {
        InsertedChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for InsertedChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        InsertedChild::Output(builder.build())
    }
}
impl From<Paragraph> for InsertedChild {
    fn from(child: Paragraph) -> Self {
        InsertedChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for InsertedChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        InsertedChild::Paragraph(builder.build())
    }
}
impl From<Picture> for InsertedChild {
    fn from(child: Picture) -> Self {
        InsertedChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for InsertedChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        InsertedChild::Picture(builder.build())
    }
}
impl From<Preformatted> for InsertedChild {
    fn from(child: Preformatted) -> Self {
        InsertedChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for InsertedChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        InsertedChild::Preformatted(builder.build())
    }
}
impl From<Progress> for InsertedChild {
    fn from(child: Progress) -> Self {
        InsertedChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for InsertedChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        InsertedChild::Progress(builder.build())
    }
}
impl From<Quote> for InsertedChild {
    fn from(child: Quote) -> Self {
        InsertedChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for InsertedChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        InsertedChild::Quote(builder.build())
    }
}
impl From<Ruby> for InsertedChild {
    fn from(child: Ruby) -> Self {
        InsertedChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for InsertedChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        InsertedChild::Ruby(builder.build())
    }
}
impl From<RubyParenthesis> for InsertedChild {
    fn from(child: RubyParenthesis) -> Self {
        InsertedChild::RubyParenthesis(child)
    }
}
impl From<builders::RubyParenthesisBuilder> for InsertedChild {
    fn from(builder: builders::RubyParenthesisBuilder) -> Self {
        InsertedChild::RubyParenthesis(builder.build())
    }
}
impl From<RubyText> for InsertedChild {
    fn from(child: RubyText) -> Self {
        InsertedChild::RubyText(child)
    }
}
impl From<builders::RubyTextBuilder> for InsertedChild {
    fn from(builder: builders::RubyTextBuilder) -> Self {
        InsertedChild::RubyText(builder.build())
    }
}
impl From<Sample> for InsertedChild {
    fn from(child: Sample) -> Self {
        InsertedChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for InsertedChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        InsertedChild::Sample(builder.build())
    }
}
impl From<Script> for InsertedChild {
    fn from(child: Script) -> Self {
        InsertedChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for InsertedChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        InsertedChild::Script(builder.build())
    }
}
impl From<Search> for InsertedChild {
    fn from(child: Search) -> Self {
        InsertedChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for InsertedChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        InsertedChild::Search(builder.build())
    }
}
impl From<Section> for InsertedChild {
    fn from(child: Section) -> Self {
        InsertedChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for InsertedChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        InsertedChild::Section(builder.build())
    }
}
impl From<Select> for InsertedChild {
    fn from(child: Select) -> Self {
        InsertedChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for InsertedChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        InsertedChild::Select(builder.build())
    }
}
impl From<Slot> for InsertedChild {
    fn from(child: Slot) -> Self {
        InsertedChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for InsertedChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        InsertedChild::Slot(builder.build())
    }
}
impl From<Small> for InsertedChild {
    fn from(child: Small) -> Self {
        InsertedChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for InsertedChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        InsertedChild::Small(builder.build())
    }
}
impl From<Source> for InsertedChild {
    fn from(child: Source) -> Self {
        InsertedChild::Source(child)
    }
}
impl From<builders::SourceBuilder> for InsertedChild {
    fn from(builder: builders::SourceBuilder) -> Self {
        InsertedChild::Source(builder.build())
    }
}
impl From<Span> for InsertedChild {
    fn from(child: Span) -> Self {
        InsertedChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for InsertedChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        InsertedChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for InsertedChild {
    fn from(child: StrikeThrough) -> Self {
        InsertedChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for InsertedChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        InsertedChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for InsertedChild {
    fn from(child: Strong) -> Self {
        InsertedChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for InsertedChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        InsertedChild::Strong(builder.build())
    }
}
impl From<Style> for InsertedChild {
    fn from(child: Style) -> Self {
        InsertedChild::Style(child)
    }
}
impl From<builders::StyleBuilder> for InsertedChild {
    fn from(builder: builders::StyleBuilder) -> Self {
        InsertedChild::Style(builder.build())
    }
}
impl From<SubScript> for InsertedChild {
    fn from(child: SubScript) -> Self {
        InsertedChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for InsertedChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        InsertedChild::SubScript(builder.build())
    }
}
impl From<Summary> for InsertedChild {
    fn from(child: Summary) -> Self {
        InsertedChild::Summary(child)
    }
}
impl From<builders::SummaryBuilder> for InsertedChild {
    fn from(builder: builders::SummaryBuilder) -> Self {
        InsertedChild::Summary(builder.build())
    }
}
impl From<SupScript> for InsertedChild {
    fn from(child: SupScript) -> Self {
        InsertedChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for InsertedChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        InsertedChild::SupScript(builder.build())
    }
}
impl From<Table> for InsertedChild {
    fn from(child: Table) -> Self {
        InsertedChild::Table(child)
    }
}
impl From<builders::TableBuilder> for InsertedChild {
    fn from(builder: builders::TableBuilder) -> Self {
        InsertedChild::Table(builder.build())
    }
}
impl From<TableBody> for InsertedChild {
    fn from(child: TableBody) -> Self {
        InsertedChild::TableBody(child)
    }
}
impl From<builders::TableBodyBuilder> for InsertedChild {
    fn from(builder: builders::TableBodyBuilder) -> Self {
        InsertedChild::TableBody(builder.build())
    }
}
impl From<TableCell> for InsertedChild {
    fn from(child: TableCell) -> Self {
        InsertedChild::TableCell(child)
    }
}
impl From<builders::TableCellBuilder> for InsertedChild {
    fn from(builder: builders::TableCellBuilder) -> Self {
        InsertedChild::TableCell(builder.build())
    }
}
impl From<TableColumn> for InsertedChild {
    fn from(child: TableColumn) -> Self {
        InsertedChild::TableColumn(child)
    }
}
impl From<builders::TableColumnBuilder> for InsertedChild {
    fn from(builder: builders::TableColumnBuilder) -> Self {
        InsertedChild::TableColumn(builder.build())
    }
}
impl From<TableColumnGroup> for InsertedChild {
    fn from(child: TableColumnGroup) -> Self {
        InsertedChild::TableColumnGroup(child)
    }
}
impl From<builders::TableColumnGroupBuilder> for InsertedChild {
    fn from(builder: builders::TableColumnGroupBuilder) -> Self {
        InsertedChild::TableColumnGroup(builder.build())
    }
}
impl From<TableFoot> for InsertedChild {
    fn from(child: TableFoot) -> Self {
        InsertedChild::TableFoot(child)
    }
}
impl From<builders::TableFootBuilder> for InsertedChild {
    fn from(builder: builders::TableFootBuilder) -> Self {
        InsertedChild::TableFoot(builder.build())
    }
}
impl From<TableHead> for InsertedChild {
    fn from(child: TableHead) -> Self {
        InsertedChild::TableHead(child)
    }
}
impl From<builders::TableHeadBuilder> for InsertedChild {
    fn from(builder: builders::TableHeadBuilder) -> Self {
        InsertedChild::TableHead(builder.build())
    }
}
impl From<TableHeader> for InsertedChild {
    fn from(child: TableHeader) -> Self {
        InsertedChild::TableHeader(child)
    }
}
impl From<builders::TableHeaderBuilder> for InsertedChild {
    fn from(builder: builders::TableHeaderBuilder) -> Self {
        InsertedChild::TableHeader(builder.build())
    }
}
impl From<TableRow> for InsertedChild {
    fn from(child: TableRow) -> Self {
        InsertedChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for InsertedChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        InsertedChild::TableRow(builder.build())
    }
}
impl From<Template> for InsertedChild {
    fn from(child: Template) -> Self {
        InsertedChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for InsertedChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        InsertedChild::Template(builder.build())
    }
}
impl From<TextArea> for InsertedChild {
    fn from(child: TextArea) -> Self {
        InsertedChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for InsertedChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        InsertedChild::TextArea(builder.build())
    }
}
impl From<Time> for InsertedChild {
    fn from(child: Time) -> Self {
        InsertedChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for InsertedChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        InsertedChild::Time(builder.build())
    }
}
impl From<Title> for InsertedChild {
    fn from(child: Title) -> Self {
        InsertedChild::Title(child)
    }
}
impl From<builders::TitleBuilder> for InsertedChild {
    fn from(builder: builders::TitleBuilder) -> Self {
        InsertedChild::Title(builder.build())
    }
}
impl From<Track> for InsertedChild {
    fn from(child: Track) -> Self {
        InsertedChild::Track(child)
    }
}
impl From<builders::TrackBuilder> for InsertedChild {
    fn from(builder: builders::TrackBuilder) -> Self {
        InsertedChild::Track(builder.build())
    }
}
impl From<Underline> for InsertedChild {
    fn from(child: Underline) -> Self {
        InsertedChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for InsertedChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        InsertedChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for InsertedChild {
    fn from(child: UnorderedList) -> Self {
        InsertedChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for InsertedChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        InsertedChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for InsertedChild {
    fn from(child: Variable) -> Self {
        InsertedChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for InsertedChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        InsertedChild::Variable(builder.build())
    }
}
impl From<Video> for InsertedChild {
    fn from(child: Video) -> Self {
        InsertedChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for InsertedChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        InsertedChild::Video(builder.build())
    }
}
impl From<WordBreak> for InsertedChild {
    fn from(child: WordBreak) -> Self {
        InsertedChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for InsertedChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        InsertedChild::WordBreak(builder.build())
    }
}
impl std::fmt::Debug for InsertedChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsertedChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Address(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Article(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Base(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Body(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Button(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Caption(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Code(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Data(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::DescriptionDetails(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::DescriptionTerm(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Details(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Division(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::FieldSetLegend(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::FigureCaption(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Form(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Head(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Header(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Html(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Image(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Input(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Label(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Link(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Main(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Map(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Object(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Option(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::OptionGroup(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Output(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::RubyParenthesis(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::RubyText(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Script(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Search(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Section(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Select(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Small(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Source(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Span(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Style(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Summary(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Table(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::TableBody(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::TableCell(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::TableColumn(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::TableColumnGroup(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::TableFoot(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::TableHead(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::TableHeader(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Template(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Time(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Title(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Track(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::Video(child) => std::fmt::Debug::fmt(child, f),
            InsertedChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for InsertedChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsertedChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Address(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Article(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Aside(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Audio(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Base(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            InsertedChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            InsertedChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Body(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Bold(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Button(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Caption(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Cite(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Code(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Custom(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Data(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::DataList(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Definition(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::DescriptionDetails(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::DescriptionTerm(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Details(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Division(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Embed(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::FieldSetLegend(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Figure(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::FigureCaption(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Footer(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Form(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Head(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Header(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Html(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Image(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Input(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Italic(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Label(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Link(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Main(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Map(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Mark(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Menu(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Meter(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Object(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Option(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::OptionGroup(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Output(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Picture(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Progress(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Quote(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::RubyParenthesis(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::RubyText(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Sample(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Script(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Search(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Section(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Select(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Slot(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Small(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Source(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Span(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Strong(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Style(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Summary(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Table(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::TableBody(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::TableCell(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::TableColumn(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::TableColumnGroup(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::TableFoot(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::TableHead(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::TableHeader(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Template(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Time(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Title(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Track(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Underline(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Variable(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::Video(child) => std::fmt::Display::fmt(child, f),
            InsertedChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
