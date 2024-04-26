import { StyleSheet, Text, TextInput, View } from "react-native";
import Layout from "../../components/Layout";
import TopBar from "../../components/TopBar";
import Button from "../../components/Button";
import { useEffect, useState } from "react";
import { Device, addDevice, getDevices, removeDevice } from "../../lib/storage";

export default function settings() {
  const [showForm, setShowForm] = useState(false);

  const [existingDevices, setExistingDevices] = useState<Device[]>();
  useEffect(() => {
    (async () => {
      setExistingDevices(await getDevices());
    })();
  }, []);

  const [nameText, setNameText] = useState<string>();
  const [rgb, setRgb] = useState<[number | undefined, number | undefined, number | undefined]>([
    undefined,
    undefined,
    undefined,
  ]);
  const [ipText, setIpText] = useState<string>();
  const [channelText, setChannelText] = useState<number>();

  async function saveDevice() {
    if (!nameText || !ipText || !rgb || !rgb[0] || !rgb[1] || !rgb[2] || !channelText) return;

    await addDevice({
      name: nameText,
      color: rgb as any,
      ip: ipText,
      channel: channelText,
    });

    setExistingDevices(await getDevices());
    setShowForm(false);
  }

  async function handleDelete(index: number) {
    removeDevice(index);
    setExistingDevices(await getDevices());
  }

  return (
    <Layout>
      <View style={styles.container}>
        <TopBar showBack />
        <View style={{ flexDirection: "row", justifyContent: "space-between", alignItems: "center", marginBottom: 16 }}>
          <Text style={{ color: "white", fontSize: 16, fontWeight: "600" }}>Naprave</Text>
          <Button onPress={() => setShowForm(true)}>Dodaj</Button>
        </View>
        {showForm ? (
          <View style={styles.formWrapper}>
            <TextInput
              value={nameText}
              onChangeText={(val) => setNameText(val)}
              style={styles.input}
              placeholder="ime naprave"
              placeholderTextColor="#707070"
            />
            <View style={{ flexDirection: "row", gap: 8 }}>
              <TextInput
                value={rgb?.[0]?.toString()}
                onChangeText={(val) => setRgb([val ? parseInt(val) : undefined, rgb?.[1], rgb?.[2]])}
                keyboardType="numeric"
                style={styles.input}
                placeholder="R"
                placeholderTextColor="#707070"
              />
              <TextInput
                value={rgb?.[1]?.toString()}
                onChangeText={(val) => setRgb([rgb?.[0], val ? parseInt(val) : undefined, rgb?.[2]])}
                keyboardType="numeric"
                style={styles.input}
                placeholder="G"
                placeholderTextColor="#707070"
              />
              <TextInput
                value={rgb?.[2]?.toString()}
                onChangeText={(val) => setRgb([rgb?.[0], rgb?.[1], val ? parseInt(val) : undefined])}
                keyboardType="numeric"
                style={styles.input}
                placeholder="B"
                placeholderTextColor="#707070"
              />
            </View>
            <TextInput
              value={ipText}
              onChangeText={(val) => setIpText(val)}
              style={styles.input}
              placeholder="IP"
              placeholderTextColor="#707070"
            />
            <TextInput
              keyboardType="numeric"
              value={channelText ? channelText.toString() : undefined}
              onChangeText={(val) => (val ? setChannelText(parseInt(val)) : undefined)}
              style={styles.input}
              placeholder="channel"
              placeholderTextColor="#707070"
            />
            <View style={{ marginTop: 8, alignItems: "center" }}>
              <Button onPress={saveDevice}>Shrani</Button>
            </View>
          </View>
        ) : null}
        {existingDevices?.length! > 0 ? (
          existingDevices?.map((x, i) => (
            <View
              key={x.name}
              style={{
                flexDirection: "row",
                justifyContent: "space-between",
                alignItems: "flex-start",
                marginBottom: 8,
              }}
            >
              <View key={x.name}>
                <Text style={styles.whiteTextTitle}>{x.name}</Text>
                <Text style={styles.whiteText}>
                  (R {x.color[0]}) (G {x.color[1]}) (B {x.color[2]})
                </Text>
                <Text style={styles.whiteText}>IP: {x.ip}</Text>
                <Text style={styles.whiteText}>Channel: {x.channel}</Text>
              </View>
              <Button
                onPress={() => {
                  handleDelete(i);
                }}
              >
                {" "}
                &times;{" "}
              </Button>
            </View>
          ))
        ) : (
          <Text style={styles.whiteTextTitle}>Ni naprav</Text>
        )}
      </View>
    </Layout>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    paddingHorizontal: 8,
  },
  formWrapper: {
    backgroundColor: "#202020",
    borderRadius: 8,
    padding: 8,
    gap: 4,
  },
  input: {
    backgroundColor: "#101010",
    borderWidth: 1,
    borderColor: "#222222",
    borderRadius: 8,
    paddingHorizontal: 8,
    paddingVertical: 4,
    color: "#ffffff",
  },
  whiteTextTitle: {
    color: "#ffffff",
    fontWeight: "bold",
  },
  whiteText: {
    color: "#ffffff",
  },
});
