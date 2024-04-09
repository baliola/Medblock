import SecondaryButton from '@components/button/SecondaryButton';
import GeneralImage from '@components/image/GeneralImage';
import ModalDialog from '@components/modal/ModalDialog';
import InfoItem from '@components/partial/InfoItem';
import TextPrimary from '@components/text/TextPrimary';
import AppBarWithIcon from '@components/top_bar/AppBarWithIcon';
import Images from '@constants/images';
import {RootStackParamList} from '@constants/routes';
import Strings from '@constants/strings';
import Scaffold from '@layouts/Scaffold';
import {StackActions, useNavigation} from '@react-navigation/native';
import {NativeStackNavigationProp} from '@react-navigation/native-stack';
import {styled} from 'nativewind';
import React, {useState} from 'react';
import {View} from 'react-native';

const StyledView = styled(View);

const SettingScreen = () => {
  const navigation =
    useNavigation<NativeStackNavigationProp<RootStackParamList>>();
  const [showAlert, setShowAlert] = useState<boolean>(false);

  const handleLogout = () => {
    setShowAlert(false);
    navigation.dispatch(StackActions.replace('Login'));
  };

  return (
    <Scaffold
      topBar={<AppBarWithIcon />}
      bottomChild={
        <StyledView className="p-6">
          <SecondaryButton
            child={
              <StyledView className="flex flex-row justify-center space-x-2 items-center">
                <GeneralImage url={Images.logout} size={24} />
                <TextPrimary
                  text={Strings.label.logout}
                  classStyle="text-gray-800"
                  isBold
                />
              </StyledView>
            }
            onPress={() => {
              setShowAlert(true);
            }}
          />
        </StyledView>
      }>
      <StyledView className="items-center mt-10">
        <GeneralImage size={100} url={Images.dummyProfile} />
        <StyledView className="w-full px-6 mt-10">
          <InfoItem data="I Wayan Aryadi" icon={Images.person} />
          <InfoItem data="24 Mei 2000" icon={Images.birthDate} />
          <InfoItem data="Arnijay@gmail.com" icon={Images.email} />

          <InfoItem
            data="Jl. Sangalangit Gang Merpati No.46, Denpasar, Bali"
            icon={Images.address}
            classStyle="pr-8"
          />
        </StyledView>
        <ModalDialog
          onRightTap={() => {
            handleLogout();
          }}
          labelCancel={Strings.label.no}
          labelRight={Strings.label.yes}
          show={showAlert}
          onClose={() => setShowAlert(false)}
          title={Strings.label.alert}
          child={
            <StyledView className="px-4">
              <TextPrimary
                classStyle="text-gray-800 text-center"
                text={Strings.message.logout}
              />
            </StyledView>
          }
        />
      </StyledView>
    </Scaffold>
  );
};

export default SettingScreen;
