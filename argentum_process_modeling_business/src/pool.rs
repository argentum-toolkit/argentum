// use argentum_standard_business::data_type::id::Id;
//
// pub trait Node {
//     fn id(&self) -> &Id;
// }
//
// pub struct Process<'a> {
//     active_node: &'a dyn Node,
// }
//
// //Swimlanes (pool or lane)
// pub struct Pool<'a> {
//     lanes: Vec<Lane>,
//     connects: Vec<&'a dyn ConnectingObject>
// }
//
// impl<'a> Pool<'a> {
//     fn new(lanes: Vec<Lane>, connects: Vec<&'a dyn ConnectingObject>) -> Pool<'a> {
//         Pool { lanes, connects }
//     }
// }
//
// pub struct Lane {
//     actor: String, //TODO implement actor type
//     // activities: Vec<&'a dyn FlowObject>,
// }
//
// //Connecting objects (sequence flow, message flow, association)
// trait ConnectingObject {
//
// }
//
// pub struct FromTo<'a> {
//     from: &'a dyn Node,
//     to: &'a dyn Node,
// }
//
// pub struct SequenceFlow<'a> {
//     from_to: &'a FromTo<'a>
// }
//
// impl<'a> ConnectingObject for SequenceFlow<'a> {
// }
//
// impl <'a> SequenceFlow<'a> {
//     fn new(from_to: &'a FromTo) -> SequenceFlow<'a> {
//         SequenceFlow { from_to }
//     }
// }
//
// //Flow objects (events, activities, gateways)
// trait FlowObject: Node {
// }
//
//
// trait Activity: FlowObject {}
//
//
// pub struct UserTask {
//     id: Id,
// }
//
// impl FlowObject for UserTask {}
//
// impl Activity for UserTask {}
//
// impl Node for UserTask {
//     fn id(&self) -> &Id {
//         &self.id
//     }
// }
//
// impl UserTask {
//     fn new(id: Id) -> UserTask {
//         UserTask { id }
//     }
// }
//
//
// pub struct Start {
//     id: Id,
// }
//
// impl Start {
//     fn new(id: Id) -> Start {
//         Start { id }
//     }
// }
//
// impl Node for Start {
//     fn id(&self) -> &Id {
//         &self.id
//     }
// }
//
// pub struct End {
//     id: Id,
// }
//
// impl End {
//     fn new(id: Id) -> End {
//         End { id }
//     }
// }
//
//
// impl Node for End {
//     fn id(&self) -> &Id {
//         &self.id
//     }
// }
//
//
//
// #[cfg(test)]
// mod tests {
//     use crate::pool::{Pool, Start, End, SequenceFlow, FromTo, UserTask};
//     use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
//     use argentum_standard_business::data_type::id::IdFactory;
//
//     #[test]
//     fn test_pools() {
//         let id_factory = IdFactoryMock::new();
//         let start = Start::new(id_factory.create());
//         let send_new_password = UserTask::new(id_factory.create());
//
//         let end = End::new(id_factory.create());
//
//         let sf1 = SequenceFlow::new(&FromTo {from: &start, to: &send_new_password });
//         let sf1 = SequenceFlow::new(&FromTo {from: &send_new_password, to: &end });
//
//         let pool = Pool::new(Vec::new(), Vec::new());
//
//     }
// }