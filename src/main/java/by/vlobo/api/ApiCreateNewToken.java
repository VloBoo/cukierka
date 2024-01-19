package by.vlobo.api;

import java.time.ZonedDateTime;

import org.json.JSONObject;

import by.vlobo.App;
import by.vlobo.Database;
import by.vlobo.IApiProcessor;
import by.vlobo.Tools;

public class ApiCreateNewToken implements IApiProcessor {

    @Override
    public JSONObject process(JSONObject message, App instance, String user) {
        Database database = instance.getDatabase();
        String username = message.getString("username");
        String password = Tools.hashPassword(message.getString("password"));
        if (database.checkPassword(username, password) != null) {
            String id = database.getUserInfoByUsername(username).getString("id");
            String dateCreation = Tools.formatDateTime(ZonedDateTime.now());
            String dateExpires = Tools.formatDateTime(ZonedDateTime.now().plusDays(30));
            JSONObject other = new JSONObject();
            other.put("date of creation", dateCreation);
            JSONObject jsonObject = database.addToken(Tools.randomUUID2(), id, dateExpires, other);
            if (jsonObject == null) {
                return IApiProcessor.CODE_500_INTERNAL_SERVER_ERROR;
            }
            return Tools.addJsonObject(IApiProcessor.CODE_200_OK, jsonObject);
        } else {
            return Tools.addJsonObject(IApiProcessor.CODE_200_OK,
                    new JSONObject().put("result", "nope"));
        }
    }
}
