import { useEffect, useRef, useState } from "react";
import { StyleSheet, Text, View } from "react-native";
import Layout from "../../components/Layout";
import TopBar from "../../components/TopBar";
import { Device, getDevices } from "../../lib/storage";
import Button from "../../components/Button";
import UdpSocket from "react-native-udp";

const PORT = 50000;

export default function IndexPage() {
  const sock = useRef<any>();

  function setColor(ip: string, channel: number, red: number, green: number, blue: number) {
    const messageData = new Uint8Array(4);
    messageData[0] = channel;
    messageData[1] = red;
    messageData[2] = green;
    messageData[3] = blue;

    sock.current.send(messageData, 0, messageData.length, PORT, ip);
  }

  function onDevice(device: Device) {
    setColor(device.ip, device.channel, device.color[0], device.color[1], device.color[2]);
  }

  function offDevice(device: Device) {
    setColor(device.ip, device.channel, 0, 0, 0);
  }

  const [devices, setDevices] = useState<Device[]>();

  useEffect(() => {
    const socket = UdpSocket.createSocket({ type: "udp4", reusePort: true });
    socket.bind(PORT);
    sock.current = socket;

    (async () => {
      setDevices(await getDevices());
    })();
  }, []);

  return (
    <Layout>
      <View style={styles.container}>
        <TopBar />
        <View style={styles.buttonsContainer}>
          {devices?.map((device) => (
            <View key={device.name} style={styles.buttonsInnerContainer}>
              <Text style={{ color: "white", fontSize: 18, fontWeight: "600" }}>{device.name}</Text>
              <View style={{ flexDirection: "row", gap: 4 }}>
                <Button
                  style={{
                    backgroundColor: `rgb(${device.color[0]}, ${device.color[1]}, ${device.color[2]})`,
                    flex: 1,
                    paddingVertical: 12,
                  }}
                  textStyle={{ color: "black", fontWeight: "800" }}
                  onPress={() => onDevice(device)}
                >
                  ON
                </Button>
                <Button
                  style={{
                    backgroundColor: `rgb(${device.color[0]}, ${device.color[1]}, ${device.color[2]})`,
                    flex: 1,
                    paddingVertical: 12,
                  }}
                  textStyle={{ color: "black", fontWeight: "800" }}
                  onPress={() => offDevice(device)}
                >
                  OFF
                </Button>
              </View>
            </View>
          ))}
        </View>
      </View>
    </Layout>
  );
}

const styles = StyleSheet.create({
  buttonsContainer: {
    alignItems: "center",
    justifyContent: "center",
    gap: 32,
    marginTop: 32,
  },
  buttonsInnerContainer: {
    width: "50%",
    gap: 4,
  },
  container: {
    flex: 1,
    paddingHorizontal: 8,
  },
});
