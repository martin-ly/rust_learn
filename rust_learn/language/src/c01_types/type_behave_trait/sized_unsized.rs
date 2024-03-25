

//! T 默认是实现了sized的所有基础类型
//! sized 即编译期就明确了内存表示的大小
fn f1<T>(){}
fn f2<T:sized>(){}

//！ ?Sized就表示UnSized类型
//！ UnSized表示编译期间 无法明确其类型的内存大小 由运行期管理
fn f3<T:?sized>(){}

