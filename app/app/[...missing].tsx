import { StyleSheet, Text, View } from "react-native";

export default function MissingScreen() {
  return (
    <View style={styles.container}>
      <Text style={styles.text}>This screen does not exist!</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: "#fff",
    justifyContent: "center",
    alignItems: 'center',
  },
  text: {
    color: "#000",
  },
});
