import FileAndCameraBottomSheet from '@components/bottom_sheet/FileAndCameraBottomSheet';
import FileButton from '@components/button/FileButton';
import PrimaryButton from '@components/button/PrimaryButton';
import GeneralImage from '@components/image/GeneralImage';
import InputCheck from '@components/input/InputCheck';
import InputText from '@components/input/InputText';
import TextPrimary from '@components/text/TextPrimary';
import Images from '@constants/images';
import {RootStackParamList} from '@constants/routes';
import Strings from '@constants/strings';
import Scaffold from '@layouts/Scaffold';
import {useNavigation} from '@react-navigation/native';
import {NativeStackNavigationProp} from '@react-navigation/native-stack';
import {styled} from 'nativewind';
import React, {useState} from 'react';
import {ScrollView, View} from 'react-native';

const StyledView = styled(View);

const FillPersonalInformationScreen = () => {
  const navigation =
    useNavigation<NativeStackNavigationProp<RootStackParamList>>();
  const [visible, setVisible] = useState<boolean>(false);
  const [fileLoading, setFileLoading] = useState<boolean>(false);
  const [uri, setUri] = useState<string | null>(null);
  const [file, setFile] = useState<File | null>(null);

  const handleSubmit = () => {
    navigation.navigate('Verified');
  };

  return (
    <Scaffold
      bottomChild={
        <StyledView className="px-8 py-4">
          <PrimaryButton
            label={Strings.label.submit}
            onPress={() => {
              handleSubmit();
            }}
            classStyle="mt-4"
          />
        </StyledView>
      }>
      <ScrollView showsVerticalScrollIndicator={false}>
        <StyledView className="w-screen flex flex-col justify-between items-center mb-72">
          <GeneralImage size={120} url={Images.logo2} classStyle="mt-8" />
          <StyledView className="w-full p-6">
            <TextPrimary
              text={Strings.message.verifyYourId}
              classStyle="text-gray-800 mt-4 mb-6"
              isBold
            />
            <InputText
              value=""
              label={Strings.label.fullname}
              onChange={e => {}}
              classStyle="mb-4"
            />
            <InputText
              value=""
              label={Strings.label.validIdNumber}
              keyboardType="number-pad"
              onChange={e => {}}
              classStyle="mb-4"
            />
            <InputText
              value=""
              label={Strings.label.address}
              onChange={e => {}}
              numberOfLine={5}
              classStyle="mb-4"
            />
            <InputText
              value=""
              label={Strings.label.phoneNumber}
              keyboardType="number-pad"
              onChange={e => {}}
              classStyle="mb-4"
            />

            <FileButton
              loading={fileLoading}
              file={file}
              uri={uri}
              onPress={() => {
                setVisible(true);
              }}
            />

            <InputCheck
              url="https://baliola.com/"
              label={Strings.message.agreeTerm}
              onChange={e => {}}
              value={false}
            />
          </StyledView>
          <FileAndCameraBottomSheet
            setLoading={value => {
              setFileLoading(value);
            }}
            getUri={uriFile => {
              setUri(uriFile);
            }}
            getFileFromLibrary={doc => {
              setFile(doc as File);
            }}
            getFileFromCamera={doc => {
              setFile(doc as File);
            }}
            onClose={() => {
              setVisible(false);
            }}
            visible={visible}
          />
        </StyledView>
      </ScrollView>
    </Scaffold>
  );
};

export default FillPersonalInformationScreen;
