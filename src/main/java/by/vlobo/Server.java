package by.vlobo;

import com.sun.net.httpserver.HttpServer;

import by.vlobo.handlers.ApiHandler;
import by.vlobo.handlers.CdnHandler;
import by.vlobo.handlers.WwwHandler;

import java.io.IOException;
import java.net.InetSocketAddress;

public class Server {

    private App app;

    public Server(App app) {
        this.app = app;
        try {
            HttpServer server = HttpServer.create(new InetSocketAddress(8080), 0);
            server.createContext("/api", new ApiHandler(app));
            server.createContext("/cdn", new CdnHandler(".", app));
            server.createContext("/", new WwwHandler(".", app));
            server.setExecutor(null);
            server.start();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}