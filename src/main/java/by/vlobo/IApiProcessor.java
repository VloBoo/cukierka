package by.vlobo;

import org.json.JSONObject;

public interface IApiProcessor {

    public static final JSONObject CODE_200_OK = new JSONObject().put("code", 200).put("status", "OK");
    //public static final JSONObject CODE_201_CREATED = new JSONObject().put("code", 201).put("status", "Created");
    //public static final JSONObject CODE_204_NO_CONTENT = new JSONObject().put("code", 204).put("status", "No Content");

    //public static final JSONObject CODE_301_MOVED_PERMANENTLY = new JSONObject().put("code", 301).put("status", "Moved Permanently");
    //public static final JSONObject CODE_302_FOUND = new JSONObject().put("code", 302).put("status", "Found");
    //public static final JSONObject CODE_304_NOT_MODIFIED = new JSONObject().put("code", 304).put("status", "Not Modified");

    public static final JSONObject CODE_400_BAD_REQUEST = new JSONObject().put("code", 400).put("status", "Bad Request");
    public static final JSONObject CODE_401_UNAUTHORIZED = new JSONObject().put("code", 401).put("status", "Unauthorized");
    public static final JSONObject CODE_403_FORBIDDEN = new JSONObject().put("code", 403).put("status", "Forbidden");
    public static final JSONObject CODE_404_NOT_FOUND = new JSONObject().put("code", 404).put("status", "Not Found");
    //public static final JSONObject CODE_405_METHOD_NOT_ALLOWED = new JSONObject().put("code", 405).put("status", "Method Not Allowed");

    public static final JSONObject CODE_500_INTERNAL_SERVER_ERROR = new JSONObject().put("code", 500).put("status", "Internal Server Error");
    public static final JSONObject CODE_501_NOT_IMPLEMENTED = new JSONObject().put("code", 501).put("status", "Not Implemented");
    public static final JSONObject CODE_503_SERVICE_UNAVAILABLE = new JSONObject().put("code", 503).put("status", "Service Unavailable");

    JSONObject process(JSONObject message, App instance, String user);
}
