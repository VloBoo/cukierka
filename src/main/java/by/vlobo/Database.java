package by.vlobo;

import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Statement;
import java.util.Properties;

import org.json.JSONArray;
import org.json.JSONObject;

// Этот класс должен служить прослойкой между бэком, что бы я спокойно вносил правки в бд и тут, а само приложение не трогал.
public class Database {
    private Connection connection;

    public static void preset() {
        try {
            Class.forName("org.postgresql.Driver");
        } catch (ClassNotFoundException e) {
            System.err.println("Dont found class name");
            e.printStackTrace();
        }
    }

    public Database(String user, String password, String host, String dbName) {
        try {
            String url = "jdbc:postgresql://" + host + "/" + dbName;
            Properties props = new Properties();
            props.setProperty("user", user);
            props.setProperty("password", password);
            connection = DriverManager.getConnection(url, props);
        } catch (SQLException e) {
            e.printStackTrace();
        }
    }

    public Database(String user, String password) {
        this(user, password, "localhost", "cukierka");
    }

    public JSONObject basecheck() {
        try {
            Statement st = connection.createStatement();
            ResultSet rs = st.executeQuery("SELECT \n" + //
                    "    table_name as \"Table\",\n" + //
                    "    column_name as \"Column\"\n" + //
                    "FROM \n" + //
                    "    information_schema.columns\n" + //
                    "WHERE \n" + //
                    "    table_catalog = 'cukierka'\n" + //
                    "    AND table_schema NOT LIKE 'pg_%'\n" + //
                    "    AND table_schema != 'information_schema'\n" + //
                    "ORDER BY \n" + //
                    "    table_name, ordinal_position;\n" + //
                    "");
            JSONArray jsonArray = new JSONArray();

            while (rs.next()) {
                jsonArray.put(Tools.toJSONObject(rs));
            }
            return new JSONObject().put("answer", jsonArray);
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }

    public JSONObject getUserByToken(String token) {
        try {
            PreparedStatement preparedStatement = connection
                    .prepareStatement("SELECT user_id FROM Tokens WHERE id = ?");
            preparedStatement.setString(1, token);
            ResultSet rs = preparedStatement.executeQuery();
            if (rs.next()) {
                return Tools.toJSONObject(rs);
            } else {
                return null;
            }
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }

    public JSONObject getUserInfoByUsername(String username) {
        try {
            PreparedStatement preparedStatement = connection.prepareStatement("SELECT * FROM Users WHERE username = ?");
            preparedStatement.setString(1, username);
            ResultSet rs = preparedStatement.executeQuery();
            if (rs.next()) {
                return Tools.toJSONObject(rs);
            } else {
                return null;
            }
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }

    public JSONObject getUserInfo(String user) {
        try {
            PreparedStatement preparedStatement = connection.prepareStatement("SELECT * FROM Users WHERE id = ?");
            preparedStatement.setString(1, user);
            ResultSet rs = preparedStatement.executeQuery();
            if (rs.next()) {
                return Tools.toJSONObject(rs);
            } else {
                return null;
            }
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }

    public JSONObject addUser(String id, String name, String email, JSONObject other) {
        try {
            String source = "INSERT INTO Users (id, username, email, other) VALUES ('%s', '%s', '%s', '%s');";
            String sql = String.format(source, id, name, email, other.toString());
            if (connection.createStatement().executeUpdate(sql) == 1) {
                return new JSONObject().put("result", "ok");
            } else {
                return new JSONObject().put("result", "error");
            }
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }

    public JSONObject checkPassword(String user, String password) {
        try {
            String source = "SELECT * FROM Users WHERE username = '%s' AND other->>'password' = '%s';";
            String sql = String.format(source, user, password);
            System.out.println(sql);
            if (connection.createStatement().executeQuery(sql).next()) {
                return new JSONObject().put("result", "ok");
            } else {
                return new JSONObject().put("result", "error");
            }
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }

    public JSONObject addToken(String id, String user, String expires, JSONObject other) {
        try {
            String source = "INSERT INTO Tokens VALUES ('%s', '%s', '%s', '%s');";
            String sql = String.format(source, id, user, expires, other.toString());
            System.out.println(sql);
            if (connection.createStatement().executeUpdate(sql) == 1) {
                return new JSONObject().put("result", "ok").put("token", id);
            } else {
                return new JSONObject().put("result", "error");
            }
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }
}
