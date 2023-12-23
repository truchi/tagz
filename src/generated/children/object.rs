// 🤖 This file is generated!

use crate::*;
/// The `<object>` element's children.
#[derive(Clone)]
pub enum ObjectChild {
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
impl From<Abbreviation> for ObjectChild {
    fn from(child: Abbreviation) -> Self {
        ObjectChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for ObjectChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        ObjectChild::Abbreviation(builder.build())
    }
}
impl From<Address> for ObjectChild {
    fn from(child: Address) -> Self {
        ObjectChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for ObjectChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        ObjectChild::Address(builder.build())
    }
}
impl From<Anchor> for ObjectChild {
    fn from(child: Anchor) -> Self {
        ObjectChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for ObjectChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        ObjectChild::Anchor(builder.build())
    }
}
impl From<Article> for ObjectChild {
    fn from(child: Article) -> Self {
        ObjectChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for ObjectChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        ObjectChild::Article(builder.build())
    }
}
impl From<Aside> for ObjectChild {
    fn from(child: Aside) -> Self {
        ObjectChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for ObjectChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        ObjectChild::Aside(builder.build())
    }
}
impl From<Audio> for ObjectChild {
    fn from(child: Audio) -> Self {
        ObjectChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for ObjectChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        ObjectChild::Audio(builder.build())
    }
}
impl From<Base> for ObjectChild {
    fn from(child: Base) -> Self {
        ObjectChild::Base(child)
    }
}
impl From<builders::BaseBuilder> for ObjectChild {
    fn from(builder: builders::BaseBuilder) -> Self {
        ObjectChild::Base(builder.build())
    }
}
impl From<BidirectionalIsolate> for ObjectChild {
    fn from(child: BidirectionalIsolate) -> Self {
        ObjectChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for ObjectChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        ObjectChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for ObjectChild {
    fn from(child: BidirectionalOverride) -> Self {
        ObjectChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for ObjectChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        ObjectChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for ObjectChild {
    fn from(child: BlockQuote) -> Self {
        ObjectChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for ObjectChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        ObjectChild::BlockQuote(builder.build())
    }
}
impl From<Body> for ObjectChild {
    fn from(child: Body) -> Self {
        ObjectChild::Body(child)
    }
}
impl From<builders::BodyBuilder> for ObjectChild {
    fn from(builder: builders::BodyBuilder) -> Self {
        ObjectChild::Body(builder.build())
    }
}
impl From<Bold> for ObjectChild {
    fn from(child: Bold) -> Self {
        ObjectChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for ObjectChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        ObjectChild::Bold(builder.build())
    }
}
impl From<Button> for ObjectChild {
    fn from(child: Button) -> Self {
        ObjectChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for ObjectChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        ObjectChild::Button(builder.build())
    }
}
impl From<Canvas> for ObjectChild {
    fn from(child: Canvas) -> Self {
        ObjectChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for ObjectChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        ObjectChild::Canvas(builder.build())
    }
}
impl From<Caption> for ObjectChild {
    fn from(child: Caption) -> Self {
        ObjectChild::Caption(child)
    }
}
impl From<builders::CaptionBuilder> for ObjectChild {
    fn from(builder: builders::CaptionBuilder) -> Self {
        ObjectChild::Caption(builder.build())
    }
}
impl From<Cite> for ObjectChild {
    fn from(child: Cite) -> Self {
        ObjectChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for ObjectChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        ObjectChild::Cite(builder.build())
    }
}
impl From<Code> for ObjectChild {
    fn from(child: Code) -> Self {
        ObjectChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for ObjectChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        ObjectChild::Code(builder.build())
    }
}
impl From<Custom> for ObjectChild {
    fn from(child: Custom) -> Self {
        ObjectChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for ObjectChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        ObjectChild::Custom(builder.build())
    }
}
impl From<Data> for ObjectChild {
    fn from(child: Data) -> Self {
        ObjectChild::Data(child)
    }
}
impl From<builders::DataBuilder> for ObjectChild {
    fn from(builder: builders::DataBuilder) -> Self {
        ObjectChild::Data(builder.build())
    }
}
impl From<DataList> for ObjectChild {
    fn from(child: DataList) -> Self {
        ObjectChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for ObjectChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        ObjectChild::DataList(builder.build())
    }
}
impl From<Definition> for ObjectChild {
    fn from(child: Definition) -> Self {
        ObjectChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for ObjectChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        ObjectChild::Definition(builder.build())
    }
}
impl From<Deleted> for ObjectChild {
    fn from(child: Deleted) -> Self {
        ObjectChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for ObjectChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        ObjectChild::Deleted(builder.build())
    }
}
impl From<DescriptionDetails> for ObjectChild {
    fn from(child: DescriptionDetails) -> Self {
        ObjectChild::DescriptionDetails(child)
    }
}
impl From<builders::DescriptionDetailsBuilder> for ObjectChild {
    fn from(builder: builders::DescriptionDetailsBuilder) -> Self {
        ObjectChild::DescriptionDetails(builder.build())
    }
}
impl From<DescriptionList> for ObjectChild {
    fn from(child: DescriptionList) -> Self {
        ObjectChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for ObjectChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        ObjectChild::DescriptionList(builder.build())
    }
}
impl From<DescriptionTerm> for ObjectChild {
    fn from(child: DescriptionTerm) -> Self {
        ObjectChild::DescriptionTerm(child)
    }
}
impl From<builders::DescriptionTermBuilder> for ObjectChild {
    fn from(builder: builders::DescriptionTermBuilder) -> Self {
        ObjectChild::DescriptionTerm(builder.build())
    }
}
impl From<Details> for ObjectChild {
    fn from(child: Details) -> Self {
        ObjectChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for ObjectChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        ObjectChild::Details(builder.build())
    }
}
impl From<Dialog> for ObjectChild {
    fn from(child: Dialog) -> Self {
        ObjectChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for ObjectChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        ObjectChild::Dialog(builder.build())
    }
}
impl From<Division> for ObjectChild {
    fn from(child: Division) -> Self {
        ObjectChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for ObjectChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        ObjectChild::Division(builder.build())
    }
}
impl From<Embed> for ObjectChild {
    fn from(child: Embed) -> Self {
        ObjectChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for ObjectChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        ObjectChild::Embed(builder.build())
    }
}
impl From<Emphasis> for ObjectChild {
    fn from(child: Emphasis) -> Self {
        ObjectChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for ObjectChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        ObjectChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for ObjectChild {
    fn from(child: FieldSet) -> Self {
        ObjectChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for ObjectChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        ObjectChild::FieldSet(builder.build())
    }
}
impl From<FieldSetLegend> for ObjectChild {
    fn from(child: FieldSetLegend) -> Self {
        ObjectChild::FieldSetLegend(child)
    }
}
impl From<builders::FieldSetLegendBuilder> for ObjectChild {
    fn from(builder: builders::FieldSetLegendBuilder) -> Self {
        ObjectChild::FieldSetLegend(builder.build())
    }
}
impl From<Figure> for ObjectChild {
    fn from(child: Figure) -> Self {
        ObjectChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for ObjectChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        ObjectChild::Figure(builder.build())
    }
}
impl From<FigureCaption> for ObjectChild {
    fn from(child: FigureCaption) -> Self {
        ObjectChild::FigureCaption(child)
    }
}
impl From<builders::FigureCaptionBuilder> for ObjectChild {
    fn from(builder: builders::FigureCaptionBuilder) -> Self {
        ObjectChild::FigureCaption(builder.build())
    }
}
impl From<Footer> for ObjectChild {
    fn from(child: Footer) -> Self {
        ObjectChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for ObjectChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        ObjectChild::Footer(builder.build())
    }
}
impl From<Form> for ObjectChild {
    fn from(child: Form) -> Self {
        ObjectChild::Form(child)
    }
}
impl From<builders::FormBuilder> for ObjectChild {
    fn from(builder: builders::FormBuilder) -> Self {
        ObjectChild::Form(builder.build())
    }
}
impl From<Head> for ObjectChild {
    fn from(child: Head) -> Self {
        ObjectChild::Head(child)
    }
}
impl From<builders::HeadBuilder> for ObjectChild {
    fn from(builder: builders::HeadBuilder) -> Self {
        ObjectChild::Head(builder.build())
    }
}
impl From<Header> for ObjectChild {
    fn from(child: Header) -> Self {
        ObjectChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for ObjectChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        ObjectChild::Header(builder.build())
    }
}
impl From<Heading1> for ObjectChild {
    fn from(child: Heading1) -> Self {
        ObjectChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for ObjectChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        ObjectChild::Heading1(builder.build())
    }
}
impl From<Heading2> for ObjectChild {
    fn from(child: Heading2) -> Self {
        ObjectChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for ObjectChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        ObjectChild::Heading2(builder.build())
    }
}
impl From<Heading3> for ObjectChild {
    fn from(child: Heading3) -> Self {
        ObjectChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for ObjectChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        ObjectChild::Heading3(builder.build())
    }
}
impl From<Heading4> for ObjectChild {
    fn from(child: Heading4) -> Self {
        ObjectChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for ObjectChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        ObjectChild::Heading4(builder.build())
    }
}
impl From<Heading5> for ObjectChild {
    fn from(child: Heading5) -> Self {
        ObjectChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for ObjectChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        ObjectChild::Heading5(builder.build())
    }
}
impl From<Heading6> for ObjectChild {
    fn from(child: Heading6) -> Self {
        ObjectChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for ObjectChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        ObjectChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for ObjectChild {
    fn from(child: HeadingGroup) -> Self {
        ObjectChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for ObjectChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        ObjectChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for ObjectChild {
    fn from(child: HorizontalRule) -> Self {
        ObjectChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for ObjectChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        ObjectChild::HorizontalRule(builder.build())
    }
}
impl From<Html> for ObjectChild {
    fn from(child: Html) -> Self {
        ObjectChild::Html(child)
    }
}
impl From<builders::HtmlBuilder> for ObjectChild {
    fn from(builder: builders::HtmlBuilder) -> Self {
        ObjectChild::Html(builder.build())
    }
}
impl From<Image> for ObjectChild {
    fn from(child: Image) -> Self {
        ObjectChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for ObjectChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        ObjectChild::Image(builder.build())
    }
}
impl From<InlineFrame> for ObjectChild {
    fn from(child: InlineFrame) -> Self {
        ObjectChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for ObjectChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        ObjectChild::InlineFrame(builder.build())
    }
}
impl From<Input> for ObjectChild {
    fn from(child: Input) -> Self {
        ObjectChild::Input(child)
    }
}
impl From<builders::InputBuilder> for ObjectChild {
    fn from(builder: builders::InputBuilder) -> Self {
        ObjectChild::Input(builder.build())
    }
}
impl From<Inserted> for ObjectChild {
    fn from(child: Inserted) -> Self {
        ObjectChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for ObjectChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        ObjectChild::Inserted(builder.build())
    }
}
impl From<Italic> for ObjectChild {
    fn from(child: Italic) -> Self {
        ObjectChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for ObjectChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        ObjectChild::Italic(builder.build())
    }
}
impl From<Keyboard> for ObjectChild {
    fn from(child: Keyboard) -> Self {
        ObjectChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for ObjectChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        ObjectChild::Keyboard(builder.build())
    }
}
impl From<Label> for ObjectChild {
    fn from(child: Label) -> Self {
        ObjectChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for ObjectChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        ObjectChild::Label(builder.build())
    }
}
impl From<LineBreak> for ObjectChild {
    fn from(child: LineBreak) -> Self {
        ObjectChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for ObjectChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        ObjectChild::LineBreak(builder.build())
    }
}
impl From<Link> for ObjectChild {
    fn from(child: Link) -> Self {
        ObjectChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for ObjectChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        ObjectChild::Link(builder.build())
    }
}
impl From<ListItem> for ObjectChild {
    fn from(child: ListItem) -> Self {
        ObjectChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for ObjectChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        ObjectChild::ListItem(builder.build())
    }
}
impl From<Main> for ObjectChild {
    fn from(child: Main) -> Self {
        ObjectChild::Main(child)
    }
}
impl From<builders::MainBuilder> for ObjectChild {
    fn from(builder: builders::MainBuilder) -> Self {
        ObjectChild::Main(builder.build())
    }
}
impl From<Map> for ObjectChild {
    fn from(child: Map) -> Self {
        ObjectChild::Map(child)
    }
}
impl From<builders::MapBuilder> for ObjectChild {
    fn from(builder: builders::MapBuilder) -> Self {
        ObjectChild::Map(builder.build())
    }
}
impl From<MapArea> for ObjectChild {
    fn from(child: MapArea) -> Self {
        ObjectChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for ObjectChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        ObjectChild::MapArea(builder.build())
    }
}
impl From<Mark> for ObjectChild {
    fn from(child: Mark) -> Self {
        ObjectChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for ObjectChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        ObjectChild::Mark(builder.build())
    }
}
impl From<Menu> for ObjectChild {
    fn from(child: Menu) -> Self {
        ObjectChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for ObjectChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        ObjectChild::Menu(builder.build())
    }
}
impl From<Metadata> for ObjectChild {
    fn from(child: Metadata) -> Self {
        ObjectChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for ObjectChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        ObjectChild::Metadata(builder.build())
    }
}
impl From<Meter> for ObjectChild {
    fn from(child: Meter) -> Self {
        ObjectChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for ObjectChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        ObjectChild::Meter(builder.build())
    }
}
impl From<Navigation> for ObjectChild {
    fn from(child: Navigation) -> Self {
        ObjectChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for ObjectChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        ObjectChild::Navigation(builder.build())
    }
}
impl From<NoScript> for ObjectChild {
    fn from(child: NoScript) -> Self {
        ObjectChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for ObjectChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        ObjectChild::NoScript(builder.build())
    }
}
impl From<Object> for ObjectChild {
    fn from(child: Object) -> Self {
        ObjectChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for ObjectChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        ObjectChild::Object(builder.build())
    }
}
impl From<Option> for ObjectChild {
    fn from(child: Option) -> Self {
        ObjectChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for ObjectChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        ObjectChild::Option(builder.build())
    }
}
impl From<OptionGroup> for ObjectChild {
    fn from(child: OptionGroup) -> Self {
        ObjectChild::OptionGroup(child)
    }
}
impl From<builders::OptionGroupBuilder> for ObjectChild {
    fn from(builder: builders::OptionGroupBuilder) -> Self {
        ObjectChild::OptionGroup(builder.build())
    }
}
impl From<OrderedList> for ObjectChild {
    fn from(child: OrderedList) -> Self {
        ObjectChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for ObjectChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        ObjectChild::OrderedList(builder.build())
    }
}
impl From<Output> for ObjectChild {
    fn from(child: Output) -> Self {
        ObjectChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for ObjectChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        ObjectChild::Output(builder.build())
    }
}
impl From<Paragraph> for ObjectChild {
    fn from(child: Paragraph) -> Self {
        ObjectChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for ObjectChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        ObjectChild::Paragraph(builder.build())
    }
}
impl From<Picture> for ObjectChild {
    fn from(child: Picture) -> Self {
        ObjectChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for ObjectChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        ObjectChild::Picture(builder.build())
    }
}
impl From<Preformatted> for ObjectChild {
    fn from(child: Preformatted) -> Self {
        ObjectChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for ObjectChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        ObjectChild::Preformatted(builder.build())
    }
}
impl From<Progress> for ObjectChild {
    fn from(child: Progress) -> Self {
        ObjectChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for ObjectChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        ObjectChild::Progress(builder.build())
    }
}
impl From<Quote> for ObjectChild {
    fn from(child: Quote) -> Self {
        ObjectChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for ObjectChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        ObjectChild::Quote(builder.build())
    }
}
impl From<Ruby> for ObjectChild {
    fn from(child: Ruby) -> Self {
        ObjectChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for ObjectChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        ObjectChild::Ruby(builder.build())
    }
}
impl From<RubyParenthesis> for ObjectChild {
    fn from(child: RubyParenthesis) -> Self {
        ObjectChild::RubyParenthesis(child)
    }
}
impl From<builders::RubyParenthesisBuilder> for ObjectChild {
    fn from(builder: builders::RubyParenthesisBuilder) -> Self {
        ObjectChild::RubyParenthesis(builder.build())
    }
}
impl From<RubyText> for ObjectChild {
    fn from(child: RubyText) -> Self {
        ObjectChild::RubyText(child)
    }
}
impl From<builders::RubyTextBuilder> for ObjectChild {
    fn from(builder: builders::RubyTextBuilder) -> Self {
        ObjectChild::RubyText(builder.build())
    }
}
impl From<Sample> for ObjectChild {
    fn from(child: Sample) -> Self {
        ObjectChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for ObjectChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        ObjectChild::Sample(builder.build())
    }
}
impl From<Script> for ObjectChild {
    fn from(child: Script) -> Self {
        ObjectChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for ObjectChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        ObjectChild::Script(builder.build())
    }
}
impl From<Search> for ObjectChild {
    fn from(child: Search) -> Self {
        ObjectChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for ObjectChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        ObjectChild::Search(builder.build())
    }
}
impl From<Section> for ObjectChild {
    fn from(child: Section) -> Self {
        ObjectChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for ObjectChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        ObjectChild::Section(builder.build())
    }
}
impl From<Select> for ObjectChild {
    fn from(child: Select) -> Self {
        ObjectChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for ObjectChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        ObjectChild::Select(builder.build())
    }
}
impl From<Slot> for ObjectChild {
    fn from(child: Slot) -> Self {
        ObjectChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for ObjectChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        ObjectChild::Slot(builder.build())
    }
}
impl From<Small> for ObjectChild {
    fn from(child: Small) -> Self {
        ObjectChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for ObjectChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        ObjectChild::Small(builder.build())
    }
}
impl From<Source> for ObjectChild {
    fn from(child: Source) -> Self {
        ObjectChild::Source(child)
    }
}
impl From<builders::SourceBuilder> for ObjectChild {
    fn from(builder: builders::SourceBuilder) -> Self {
        ObjectChild::Source(builder.build())
    }
}
impl From<Span> for ObjectChild {
    fn from(child: Span) -> Self {
        ObjectChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for ObjectChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        ObjectChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for ObjectChild {
    fn from(child: StrikeThrough) -> Self {
        ObjectChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for ObjectChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        ObjectChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for ObjectChild {
    fn from(child: Strong) -> Self {
        ObjectChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for ObjectChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        ObjectChild::Strong(builder.build())
    }
}
impl From<Style> for ObjectChild {
    fn from(child: Style) -> Self {
        ObjectChild::Style(child)
    }
}
impl From<builders::StyleBuilder> for ObjectChild {
    fn from(builder: builders::StyleBuilder) -> Self {
        ObjectChild::Style(builder.build())
    }
}
impl From<SubScript> for ObjectChild {
    fn from(child: SubScript) -> Self {
        ObjectChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for ObjectChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        ObjectChild::SubScript(builder.build())
    }
}
impl From<Summary> for ObjectChild {
    fn from(child: Summary) -> Self {
        ObjectChild::Summary(child)
    }
}
impl From<builders::SummaryBuilder> for ObjectChild {
    fn from(builder: builders::SummaryBuilder) -> Self {
        ObjectChild::Summary(builder.build())
    }
}
impl From<SupScript> for ObjectChild {
    fn from(child: SupScript) -> Self {
        ObjectChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for ObjectChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        ObjectChild::SupScript(builder.build())
    }
}
impl From<Table> for ObjectChild {
    fn from(child: Table) -> Self {
        ObjectChild::Table(child)
    }
}
impl From<builders::TableBuilder> for ObjectChild {
    fn from(builder: builders::TableBuilder) -> Self {
        ObjectChild::Table(builder.build())
    }
}
impl From<TableBody> for ObjectChild {
    fn from(child: TableBody) -> Self {
        ObjectChild::TableBody(child)
    }
}
impl From<builders::TableBodyBuilder> for ObjectChild {
    fn from(builder: builders::TableBodyBuilder) -> Self {
        ObjectChild::TableBody(builder.build())
    }
}
impl From<TableCell> for ObjectChild {
    fn from(child: TableCell) -> Self {
        ObjectChild::TableCell(child)
    }
}
impl From<builders::TableCellBuilder> for ObjectChild {
    fn from(builder: builders::TableCellBuilder) -> Self {
        ObjectChild::TableCell(builder.build())
    }
}
impl From<TableColumn> for ObjectChild {
    fn from(child: TableColumn) -> Self {
        ObjectChild::TableColumn(child)
    }
}
impl From<builders::TableColumnBuilder> for ObjectChild {
    fn from(builder: builders::TableColumnBuilder) -> Self {
        ObjectChild::TableColumn(builder.build())
    }
}
impl From<TableColumnGroup> for ObjectChild {
    fn from(child: TableColumnGroup) -> Self {
        ObjectChild::TableColumnGroup(child)
    }
}
impl From<builders::TableColumnGroupBuilder> for ObjectChild {
    fn from(builder: builders::TableColumnGroupBuilder) -> Self {
        ObjectChild::TableColumnGroup(builder.build())
    }
}
impl From<TableFoot> for ObjectChild {
    fn from(child: TableFoot) -> Self {
        ObjectChild::TableFoot(child)
    }
}
impl From<builders::TableFootBuilder> for ObjectChild {
    fn from(builder: builders::TableFootBuilder) -> Self {
        ObjectChild::TableFoot(builder.build())
    }
}
impl From<TableHead> for ObjectChild {
    fn from(child: TableHead) -> Self {
        ObjectChild::TableHead(child)
    }
}
impl From<builders::TableHeadBuilder> for ObjectChild {
    fn from(builder: builders::TableHeadBuilder) -> Self {
        ObjectChild::TableHead(builder.build())
    }
}
impl From<TableHeader> for ObjectChild {
    fn from(child: TableHeader) -> Self {
        ObjectChild::TableHeader(child)
    }
}
impl From<builders::TableHeaderBuilder> for ObjectChild {
    fn from(builder: builders::TableHeaderBuilder) -> Self {
        ObjectChild::TableHeader(builder.build())
    }
}
impl From<TableRow> for ObjectChild {
    fn from(child: TableRow) -> Self {
        ObjectChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for ObjectChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        ObjectChild::TableRow(builder.build())
    }
}
impl From<Template> for ObjectChild {
    fn from(child: Template) -> Self {
        ObjectChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for ObjectChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        ObjectChild::Template(builder.build())
    }
}
impl From<TextArea> for ObjectChild {
    fn from(child: TextArea) -> Self {
        ObjectChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for ObjectChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        ObjectChild::TextArea(builder.build())
    }
}
impl From<Time> for ObjectChild {
    fn from(child: Time) -> Self {
        ObjectChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for ObjectChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        ObjectChild::Time(builder.build())
    }
}
impl From<Title> for ObjectChild {
    fn from(child: Title) -> Self {
        ObjectChild::Title(child)
    }
}
impl From<builders::TitleBuilder> for ObjectChild {
    fn from(builder: builders::TitleBuilder) -> Self {
        ObjectChild::Title(builder.build())
    }
}
impl From<Track> for ObjectChild {
    fn from(child: Track) -> Self {
        ObjectChild::Track(child)
    }
}
impl From<builders::TrackBuilder> for ObjectChild {
    fn from(builder: builders::TrackBuilder) -> Self {
        ObjectChild::Track(builder.build())
    }
}
impl From<Underline> for ObjectChild {
    fn from(child: Underline) -> Self {
        ObjectChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for ObjectChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        ObjectChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for ObjectChild {
    fn from(child: UnorderedList) -> Self {
        ObjectChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for ObjectChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        ObjectChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for ObjectChild {
    fn from(child: Variable) -> Self {
        ObjectChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for ObjectChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        ObjectChild::Variable(builder.build())
    }
}
impl From<Video> for ObjectChild {
    fn from(child: Video) -> Self {
        ObjectChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for ObjectChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        ObjectChild::Video(builder.build())
    }
}
impl From<WordBreak> for ObjectChild {
    fn from(child: WordBreak) -> Self {
        ObjectChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for ObjectChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        ObjectChild::WordBreak(builder.build())
    }
}
impl std::fmt::Debug for ObjectChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Address(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Article(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Base(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Body(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Button(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Caption(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Code(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Data(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::DescriptionDetails(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::DescriptionTerm(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Details(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Division(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::FieldSetLegend(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::FigureCaption(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Form(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Head(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Header(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Html(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Image(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Input(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Label(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Link(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Main(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Map(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Object(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Option(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::OptionGroup(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Output(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::RubyParenthesis(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::RubyText(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Script(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Search(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Section(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Select(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Small(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Source(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Span(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Style(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Summary(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Table(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::TableBody(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::TableCell(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::TableColumn(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::TableColumnGroup(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::TableFoot(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::TableHead(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::TableHeader(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Template(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Time(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Title(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Track(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::Video(child) => std::fmt::Debug::fmt(child, f),
            ObjectChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for ObjectChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Address(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Article(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Aside(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Audio(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Base(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Body(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Bold(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Button(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Caption(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Cite(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Code(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Custom(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Data(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::DataList(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Definition(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::DescriptionDetails(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::DescriptionTerm(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Details(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Division(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Embed(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::FieldSetLegend(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Figure(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::FigureCaption(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Footer(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Form(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Head(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Header(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Html(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Image(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Input(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Italic(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Label(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Link(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Main(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Map(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Mark(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Menu(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Meter(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Object(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Option(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::OptionGroup(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Output(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Picture(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Progress(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Quote(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::RubyParenthesis(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::RubyText(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Sample(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Script(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Search(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Section(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Select(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Slot(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Small(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Source(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Span(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Strong(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Style(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Summary(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Table(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::TableBody(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::TableCell(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::TableColumn(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::TableColumnGroup(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::TableFoot(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::TableHead(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::TableHeader(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Template(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Time(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Title(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Track(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Underline(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Variable(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::Video(child) => std::fmt::Display::fmt(child, f),
            ObjectChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
