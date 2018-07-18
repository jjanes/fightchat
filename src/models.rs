#[derive(Queryable)]
pub struct ChatMessages {
    pub id: i:32,
    pub sender_id: i:32,
    pub recipient_id: i:32,
  
    pub message: String,
    pub body: String,
    pub published: bool,
}


//  create_table "chat_messages", force: true do |t|
//    t.integer  "sender_id",    null: false
//    t.integer  "recipient_id", null: false
//    t.string   "message",      null: false
//    t.datetime "created_at"
//    t.datetime "updated_at"
//  end

