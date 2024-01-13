package by.vlobo.api;

import java.time.ZonedDateTime;

import org.json.JSONObject;

import by.vlobo.App;
import by.vlobo.IApiProcessor;
import by.vlobo.Tools;

public class ApiRegistToken implements IApiProcessor {

    @Override
    public JSONObject process(JSONObject message, App instance) {
        message = message.getJSONObject("message");
        String username = message.getString("username");
        String password = Tools.hashPassword(message.getString("password"));
        if (instance.getDatabase().checkPassword(username, password).getString("result").equals("ok")) {
            String id = instance.getDatabase().getUserInfoByUsername(username).getString("id");
            String dateCreation = Tools.formatDateTime(ZonedDateTime.now());
            String dateExpires = Tools.formatDateTime(ZonedDateTime.now().plusDays(30));
            JSONObject other = new JSONObject();
            other.put("date of creation", dateCreation);
            return instance.getDatabase().addToken(Tools.randomUUID2(), id, dateExpires, other);
        } else {
            return new JSONObject().put("result", "nope").put("reason", "login or password is incorrect");
        }
    }
}
