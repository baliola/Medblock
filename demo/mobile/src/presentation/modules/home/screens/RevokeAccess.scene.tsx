import IconButton from '@components/button/IconButton';
import PrimaryButton from '@components/button/PrimaryButton';
import GeneralImage from '@components/image/GeneralImage';
import InputCheck from '@components/input/InputCheck';
import InputText from '@components/input/InputText';
import BasicModal from '@components/modal/BasicModal';
import ModalDialog from '@components/modal/ModalDialog';
import TextPrimary from '@components/text/TextPrimary';
import AppBarWithIcon from '@components/top_bar/AppBarWithIcon';
import Images from '@constants/images';
import {RootStackParamList} from '@constants/routes';
import Strings from '@constants/strings';
import Scaffold from '@layouts/Scaffold';
import {useNavigation} from '@react-navigation/native';
import {NativeStackNavigationProp} from '@react-navigation/native-stack';
import {styled} from 'nativewind';
import React, {useState} from 'react';
import {useTranslation} from 'react-i18next';
import {FlatList, ScrollView, View} from 'react-native';

const StyledView = styled(View);
const StyledScrollView = styled(ScrollView);

const StyledFlatListHospital = styled(FlatList as new () => FlatList<any>);

const data = [
  {
    id: 1,
    selected: true,
    title: 'Sanglah Hospital-Denpasar',
    latest: '24 Maret 2024',
    physician: 'Karyada Irawan',
  },
  {
    id: 2,
    selected: false,
    title: 'Pertamina',
    latest: '22 Maret 2024',
    physician: 'Indra Kamataru',
  },
  {
    id: 3,
    selected: false,
    title: 'DR. Soedjono Hospital',
    latest: '21 Maret 2024',
    physician: 'Samika Karamoy',
  },
];

const RevokeAccessScreen = () => {
  const {t} = useTranslation('global');
  const navigation =
    useNavigation<NativeStackNavigationProp<RootStackParamList>>();

  const [showAlert, setShowAlert] = useState<boolean>(false);
  const [showClosedEmr, setShowClosedEmr] = useState<boolean>(false);

  const handleRevoke = () => {
    setShowAlert(false);
    setShowClosedEmr(true);
  };

  const handleSendRevoke = () => {
    setShowClosedEmr(false);
    navigation.goBack();
  };

  const renderItem = ({item}: {item: any}) => (
    <StyledView className="w-full flex flex-row items-center mb-6 space-x-4">
      <InputCheck onChange={e => {}} value={item.selected} />
      <GeneralImage url={Images.hospital} size={52} />
      <StyledView>
        <TextPrimary text={item.title} classStyle="text-gray-800" isBold />
        <TextPrimary
          text={t(Strings.label.lastVisited, {
            date: item.latest,
          })}
          classStyle="text-gray-800 text-xs my-1"
        />
        <TextPrimary
          text={t(Strings.label.physician, {
            physician: item.physician,
          })}
          classStyle="text-gray-800 text-xs"
        />
      </StyledView>
    </StyledView>
  );

  return (
    <Scaffold
      topBar={
        <AppBarWithIcon
          title={Strings.message.findAndRevoke}
          child={
            <InputText
              value=""
              classStyle="w-full px-6 mt-4"
              prefix={
                <GeneralImage
                  url={Images.search}
                  size={18}
                  classStyle="self-center mr-4"
                />
              }
              suffix={
                <IconButton
                  classStyle="self-center"
                  icon={<GeneralImage url={Images.clear} size={18} />}
                  onPress={() => {}}
                />
              }
              placeholder={Strings.placeholder.search}
            />
          }
        />
      }
      bottomChild={
        <StyledView className="p-6">
          <PrimaryButton
            label={Strings.label.revokeEmr}
            onPress={() => {
              setShowAlert(true);
            }}
          />
        </StyledView>
      }>
      <>
        <StyledScrollView className="p-6" showsVerticalScrollIndicator={false}>
          <StyledFlatListHospital
            className="w-full"
            scrollEnabled={false}
            showsVerticalScrollIndicator={false}
            data={data}
            renderItem={renderItem}
            keyExtractor={item => item.id}
          />
        </StyledScrollView>
        <ModalDialog
          onRightTap={() => {
            handleRevoke();
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
                text={Strings.message.closeEmr}
              />
            </StyledView>
          }
        />
        <BasicModal
          show={showClosedEmr}
          onClose={() => {
            handleSendRevoke();
          }}
          child={
            <StyledView className="flex flex-col items-center">
              <GeneralImage url={Images.error} size={150} />
              <TextPrimary
                text={Strings.message.emrClosed}
                classStyle="text-gray-800 text-center px-10 mt-4"
                isBold
              />
            </StyledView>
          }
        />
      </>
    </Scaffold>
  );
};

export default RevokeAccessScreen;
