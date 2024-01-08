import { useEffect, useRef } from "react";
import { Button, StyleSheet, View } from "react-native";
import UdpSocket from "react-native-udp";
import Layout from "../../components/Layout";

const PORT = 50000;
const BROADCAST_ADDR = "10.255.255.255";

export default function IndexPage() {
  const sock = useRef<any>();

  function setColor(channel: string, red: number, green: number, blue: number) {
    const ch = channel.padStart(2, "0");
    const r = red.toString().padStart(3, "0");
    const g = green.toString().padStart(3, "0");
    const b = blue.toString().padStart(3, "0");

    const message = `${ch}C${r}${g}${b}`;
    sock.current.send(message, 0, message.length, PORT, BROADCAST_ADDR);
  }

  function onRacunalnik() {
    setColor("01", 255, 100, 10);
  }

  function onPostelja() {
    setColor("03", 255, 100, 10);
  }

  function offRacunalnik() {
    setColor("01", 0, 0, 0);
  }

  function offPostelja() {
    setColor("03", 0, 0, 0);
  }

  useEffect(() => {
    const socket = UdpSocket.createSocket({ type: "udp4", reusePort: true });
    socket.bind(PORT);
    sock.current = socket;
  }, []);

  return (
    <Layout>
      <View style={styles.buttonsContainer}>
        <View style={styles.buttonsInnerContainer}>
          <Button color="#f5a41b" title="ON Racunalnik" onPress={onRacunalnik} />
          <Button color="#422a01" title="OFF Racunalnik" onPress={offRacunalnik} />
        </View>
        <View style={styles.buttonsInnerContainer}>
          <Button color="#f5a41b" title="ON Postelja" onPress={onPostelja} />
          <Button color="#422a01" title="OFF Postelja" onPress={offPostelja} />
        </View>
      </View>
    </Layout>
  );
}

const styles = StyleSheet.create({
  buttonsContainer: {
    alignItems: "center",
    justifyContent: "center",
    gap: 50,
  },
  buttonsInnerContainer: {
    width: "50%",
  },
});
