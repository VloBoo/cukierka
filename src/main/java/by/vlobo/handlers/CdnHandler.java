package by.vlobo.handlers;

import com.sun.net.httpserver.HttpHandler;

import by.vlobo.App;

import com.sun.net.httpserver.HttpExchange;

import java.io.File;
import java.io.IOException;
import java.io.OutputStream;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.List;

public class CdnHandler implements HttpHandler {

    private String rootPath;
    private App app;

    public CdnHandler(String rootPath, App app) {
        this.rootPath = rootPath;
        this.app = app;
    }

    @Override
    public void handle(HttpExchange t) throws IOException {
        String path = t.getRequestURI().getPath().replaceFirst("^/cdn", "");
        String filePath = rootPath + path;

        System.out.println(path);

        File file = new File(filePath);
        if (file.isDirectory()) {
            List<String> fileNames = new ArrayList<>();
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
