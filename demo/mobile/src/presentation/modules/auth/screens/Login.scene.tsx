import BorderRoundedButton from '@components/button/BorderRoundedButton';
import PrimaryButton from '@components/button/PrimaryButton';
import TextButton from '@components/button/TextButton';
import GeneralImage from '@components/image/GeneralImage';
import InputText from '@components/input/InputText';
import TextPrimary from '@components/text/TextPrimary';
import Images from '@constants/images';
import {RootStackParamList} from '@constants/routes';
import Strings from '@constants/strings';
import Scaffold from '@layouts/Scaffold';
import {useNavigation} from '@react-navigation/native';
import {NativeStackNavigationProp} from '@react-navigation/native-stack';
import {styled} from 'nativewind';
import React from 'react';
import {ScrollView, StyleSheet, View} from 'react-native';

const StyledView = styled(View);
const StyledScrollView = styled(ScrollView);

const LoginScreen = () => {
  const navigation =
    useNavigation<NativeStackNavigationProp<RootStackParamList>>();

  const handleLogin = () => {
    navigation.navigate('Unverified');
  };

  return (
    <Scaffold>
      <StyledScrollView showsVerticalScrollIndicator={false}>
        <StyledView className="w-screen min-h-screen  pt-8 flex flex-col justify-between">
          <StyledView className="w-screen items-center">
            <GeneralImage size={180} url={Images.logo2} classStyle="mt-8" />
            <StyledView className="w-full p-6">
              <InputText
                value=""
                label={Strings.label.username}
                placeholder="your@username.com"
                onChange={e => {}}
              />
              <PrimaryButton
                label={Strings.label.continueEmail}
                onPress={() => {
                  handleLogin();
                }}
                classStyle="mt-4"
              />

              <StyledView className="flex flex-row my-4 items-center">
                <StyledView
                  className="flex-1 bg-gray-500"
                  style={styles.line2}
                />
                <TextPrimary
                  text={Strings.label.orContinue}
                  classStyle="text-xs text-gray-500 mx-2"
                  isBold
                />
                <StyledView
                  className="flex-1 bg-gray-500"
                  style={styles.line2}
                />
              </StyledView>
              <StyledView className="flex flex-row mb-4 items-center space-x-4">
                <BorderRoundedButton
                  onPress={() => {}}
                  classStyle="flex-1 p-4"
                  child={<GeneralImage size={20} url={Images.google} />}
                />
                <BorderRoundedButton
                  onPress={() => {}}
                  classStyle="flex-1 px-4"
                  child={<GeneralImage size={52} url={Images.eid} />}
                />
              </StyledView>
              <BorderRoundedButton
                onPress={() => {}}
                classStyle="flex-1 p-4"
                child={
                  <StyledView className="flex flex-row items-center space-x-4">
                    <GeneralImage size={28} url={Images.passkey} />
                    <TextPrimary
                      text={Strings.label.continuePassKey}
                      classStyle="text-gray-800"
                    />
                  </StyledView>
                }
              />
              <TextButton
                label={Strings.label.otherSign}
                onPress={() => {}}
                classStyle="self-center mt-4 text-cyan-400"
              />
            </StyledView>
          </StyledView>
          <StyledView className="flex flex-row justify-between px-8 w-full items-center absolute bottom-0">
            <StyledView className="flex flex-row space-x-4">
              <TextButton
                label={Strings.label.terms}
                classStyle="text-gray-500 text-xs"
                onPress={() => {}}
              />
              <TextButton
                label={Strings.label.privacyPolicy}
                classStyle="text-gray-500 text-xs"
                onPress={() => {}}
              />
            </StyledView>
            <GeneralImage size={40} url={Images.nfid} />
          </StyledView>
        </StyledView>
      </StyledScrollView>
    </Scaffold>
  );
};

const styles = StyleSheet.create({
  line2: {
    height: 2,
  },
});

export default LoginScreen;
