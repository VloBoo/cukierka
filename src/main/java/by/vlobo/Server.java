package by.vlobo;

import com.sun.net.httpserver.HttpServer;
import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpExchange;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import org.json.JSONException;
import org.json.JSONObject;

public class Server {

    private App app;

    public Server(App app) {
        this.app = app;
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
                System.out.println(road);

                switch (road) {
                    case "#":
                        apiHandle(t, app, vars);
                        break;
                    case "$":
                        cdnHandle(t, vars);
                        break;

                    default:
                        break;
                }

                //

                t.getResponseHeaders().set("Content-Type", "text/html; charset=UTF-8");
                t.sendResponseHeaders(200, road.getBytes().length);
                try (OutputStream os = t.getResponseBody()) {
                    os.write(road.getBytes());
                }
                //

                break;
            }
        }
        String notFoundResponse = "File not found";
        t.getResponseHeaders().set("Content-Type", "text/html; charset=UTF-8");
        t.sendResponseHeaders(404, notFoundResponse.getBytes().length);
        try (OutputStream os = t.getResponseBody()) {
            os.write(notFoundResponse.getBytes());
        }
    }

    public void wwwHandle(HttpExchange t, App instance, HashMap<String, String> vars) {
        try {
            String responseStr = "e";
            t.getResponseHeaders().set("Content-Type", "text/html; charset=UTF-8");
            t.sendResponseHeaders(202, responseStr.length());
            try (OutputStream os = t.getResponseBody()) {
                os.write(responseStr.getBytes());
            }
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
                    user = instance.getDatabase().getUserByToken(user).getString("user_id");
                }
                JSONObject message = new JSONObject(new String(t.getRequestBody().readAllBytes()));
                response = Tools.processMessage(vars.get("1"), message, instance, user);
            } catch (JSONException e) {
                response = IApiProcessor.CODE_400_BAD_REQUEST;
            }
            System.out.println(response);
            String responseStr = response.toString(4);
            t.getResponseHeaders().set("Content-Type", "application/json; charset=UTF-8");
            t.sendResponseHeaders(response.getInt("code"), responseStr.length());
            try (OutputStream os = t.getResponseBody()) {
                os.write(responseStr.getBytes());
            }
        } catch (JSONException | IOException e) {
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
            t.sendResponseHeaders(200, response.length());
            try (OutputStream os = t.getResponseBody()) {
                os.write(response.getBytes());
            }
        } else if (file.exists() && file.isFile()) {
            t.getResponseHeaders().set("Content-Type", "text/plain; charset=UTF-8");
            t.sendResponseHeaders(200, file.length());
            try (OutputStream os = t.getResponseBody()) {
                Files.copy(file.toPath(), os);
            }
        } else {
            String notFoundResponse = "File not found";
            t.sendResponseHeaders(404, notFoundResponse.length());
            try (OutputStream os = t.getResponseBody()) {
                os.write(notFoundResponse.getBytes());
            }
        }
    }
}