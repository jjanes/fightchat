table! {
    chat_messages (id) {
        id -> Integer,
        sender_id -> Integer,
        recipient_id -> Integer,
        message -> Text,
        body -> Text,
        published -> Bool,
    }
}


//  create_table "chat_messages", force: true do |t|
//    t.integer  "sender_id",    null: false
//    t.integer  "recipient_id", null: false
//    t.string   "message",      null: false
//    t.datetime "created_at"
//    t.datetime "updated_at"
//   end

