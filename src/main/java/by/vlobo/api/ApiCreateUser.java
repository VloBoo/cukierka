package by.vlobo.api;

import java.time.ZonedDateTime;
import java.util.UUID;

import org.json.JSONObject;

import by.vlobo.App;
import by.vlobo.IApiProcessor;
import by.vlobo.Tools;

public class ApiCreateUser implements IApiProcessor {

    @Override
    public JSONObject process(JSONObject message, App instance) {
        message = message.getJSONObject("message");
        String username = message.getString("username");
        String email = message.getString("email");
        String password = Tools.hashPassword(message.getString("password"));
        String dateCreation = ZonedDateTime.now().toString();
        JSONObject other = new JSONObject();
        other.put("password", password);
        other.put("date of creation", dateCreation);
        return instance.getDatabase().addUser(Tools.randomUUID2(), username, email, other);
    }
}
