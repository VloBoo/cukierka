package by.vlobo;

public class App {
    private Server server;
    private Database database;

    public App() {

        database = new Database("VloBo", " 1");

    }

    public void start() {
        server = new Server(this);
    }

    public Server getServer() {
        return server;
    }

    public Database getDatabase() {
        return database;
    }
}
