package by.vlobo;

import com.sun.net.httpserver.HttpServer;
import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpExchange;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.net.InetSocketAddress;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import org.json.JSONException;
import org.json.JSONObject;

public class Server {

    public Server(App app) {
        try {
            HttpServer server = HttpServer.create(new InetSocketAddress(8080), 0);
            server.createContext("/", new ServerHandler(app));
            server.setExecutor(null);
            server.start();
        } catch (JSONException e) {
            e.printStackTrace();
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}

class ServerHandler implements HttpHandler {
    private App app;

    public ServerHandler(App app) {
        this.app = app;
    }

    @Override
    public void handle(HttpExchange t) throws IOException {
        String path = t.getRequestURI().getPath();
        System.out.println(path);
        for (Pattern pattern : app.getRoadmap().keySet()) {

            Matcher matcher = pattern.matcher(path);

            if (matcher.matches()) {
                HashMap<String, String> vars = new HashMap<>();
                for (int i = matcher.groupCount(); i > 0; i--) {
                    // Увы, нельзя обратно получить название группы, поэтому пока такой костыль. В
                    // будущем мб заменю
                    vars.put(new String("" + i), matcher.group(i));
                }
                String road = app.getRoadmap().get(pattern);
                switch (road) {
                    case "#":
                        apiHandle(t, app, vars);
                        break;
                    case "$":
                        cdnHandle(t, vars);
                        break;

                    default:
                        wwwHandle(t, app, vars, road);
                        break;
                }
                break;
            }
        }
        String notFoundResponse = "File not found";
        t.getResponseHeaders().set("Content-Type", "text/html; charset=UTF-8");
        Tools.sendString(t, 404, notFoundResponse);
    }

    public void wwwHandle(HttpExchange t, App instance, HashMap<String, String> vars, String road) {
        try {
            String response = Tools.linkHtml(road, vars);
            t.getResponseHeaders().set("Content-Type", "text/html; charset=UTF-8");
            Tools.sendString(t, 200, response);
        } catch (JSONException | IOException e) {
            e.printStackTrace();
        }
    }

    public void apiHandle(HttpExchange t, App instance, HashMap<String, String> vars) {
        String user = t.getRequestHeaders().getFirst("Authorization");
        try {
            JSONObject response;
            try {
                if (user != null) {
                    String uuidPattern = "([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})|"
                            +
                            "([0-9a-fA-F]{32})";
                    Matcher matcher = Pattern.compile(uuidPattern).matcher(user);
                    if (matcher.find()) {
                        user = instance.getDatabase().getUserByToken(matcher.group(0)).getString("user_id");
                    } else {
                        user = null;
                    }
                }
                JSONObject message = new JSONObject(new String(t.getRequestBody().readAllBytes()));
                response = Tools.processMessage(vars.get("1"), message, instance, user);
            } catch (JSONException e) {
                response = IApiProcessor.CODE_400_BAD_REQUEST;
            }
            System.out.println(response);
            String responseStr = response.toString(4);
            t.getResponseHeaders().set("Content-Type", "application/json; charset=UTF-8");
            Tools.sendString(t, response.getInt("code"), responseStr);
        } catch (JSONException | IOException e) {
            e.printStackTrace();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void cdnHandle(HttpExchange t, HashMap<String, String> vars) throws IOException {

        String filePath = "./" + vars.get("1");

        System.out.println(filePath);

        File file = new File(filePath);
        if (file.isDirectory()) {
            ArrayList<String> fileNames = new ArrayList<>();
            for (File f : file.listFiles()) {
                fileNames.add(
                        "<a href=\"./" +
                                f.getName() +
                                (f.isDirectory() ? "/" : "") +
                                "\">" + f.getName() + "</a>");
            }

            String response = String.join("<br>", fileNames);
            Tools.sendString(t, 200, response);
        } else if (file.exists() && file.isFile()) {
            t.getResponseHeaders().set("Content-Type", "text/plain; charset=UTF-8");
            Tools.sendFile(t, 200, file);
        } else {
            String notFoundResponse = "File not found";
            Tools.sendString(t, 400, notFoundResponse);
        }
    }
}