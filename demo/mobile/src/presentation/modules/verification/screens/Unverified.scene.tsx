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

const UnverifiedScreen = () => {
  const navigation =
    useNavigation<NativeStackNavigationProp<RootStackParamList>>();

  const handleVerifyId = () => {
    navigation.navigate('FillPersonalInformation');
  };

  return (
    <Scaffold>
      <StyledView className="flex flex-col justify-center items-center mt-24 px-8">
        <GeneralImage size={120} url={Images.logo2} classStyle="mt-8" />
        <StyledView className="my-10 flex flex-col items-center">
          <TextPrimary
            text={Strings.label.welcome}
            classStyle="text-gray-800 text-xl"
            isBold
          />
          <GeneralImage
            size={120}
            url={Images.dummyProfile}
            classStyle="my-4"
          />
          <TextPrimary
            text="I Wayan Arnijayadi Supatra"
            classStyle="text-gray-800 text-xl"
            isBold
          />
        </StyledView>
        <PrimaryButton
          label={Strings.label.verifyYourId}
          onPress={() => {
            handleVerifyId();
          }}
        />
      </StyledView>
    </Scaffold>
  );
};

export default UnverifiedScreen;
