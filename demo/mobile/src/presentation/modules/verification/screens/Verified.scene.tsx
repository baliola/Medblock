import PrimaryButton from '@components/button/PrimaryButton';
import GeneralImage from '@components/image/GeneralImage';
import TextPrimary from '@components/text/TextPrimary';
import Images from '@constants/images';
import {RootStackParamList} from '@constants/routes';
import Strings from '@constants/strings';
import Scaffold from '@layouts/Scaffold';
import {useNavigation} from '@react-navigation/native';
import {NativeStackNavigationProp} from '@react-navigation/native-stack';
import {styled} from 'nativewind';
import React from 'react';
import {View} from 'react-native';

const StyledView = styled(View);

const VerifiedScreen = () => {
  const navigation =
    useNavigation<NativeStackNavigationProp<RootStackParamList>>();

  const handleVerifyId = () => {
    navigation.navigate('MainNavigation');
  };

  return (
    <Scaffold>
      <StyledView className="flex flex-col justify-center items-center px-8 h-screen">
        <StyledView className="my-10 flex flex-col items-center">
          <GeneralImage size={240} url={Images.check} classStyle="my-4" />
          <TextPrimary
            text={Strings.label.yourAccVerified}
            classStyle="text-gray-800 text-xl"
            isBold
          />
          <TextPrimary
            text={Strings.message.yourAccVerified}
            classStyle="text-gray-800 text-center px-6"
          />
        </StyledView>
        <PrimaryButton
          label={Strings.label.continue}
          onPress={() => {
            handleVerifyId();
          }}
        />
      </StyledView>
    </Scaffold>
  );
};

export default VerifiedScreen;
