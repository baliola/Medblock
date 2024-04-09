import BasicButton from '@components/button/BasicButton';
import IconLabelButton from '@components/button/IconLabelButton';
import GeneralImage from '@components/image/GeneralImage';
import TextPrimary from '@components/text/TextPrimary';
import AppBar from '@components/top_bar/AppBar';
import Images from '@constants/images';
import Strings from '@constants/strings';
import Scaffold from '@layouts/Scaffold';
import {styled} from 'nativewind';
import React from 'react';
import {FlatList, View} from 'react-native';

const StyledView = styled(View);

const StyledFlatListNotification = styled(FlatList as new () => FlatList<any>);

const data = [
  {
    id: 1,
    icon: Images.success,
    title: 'Your EMR Has Been Access',
    subtitle: 'Pertamina Hospital-Nusantara City',
    dateTime: '24 Maret 2024 - 18.00 WIB',
    physician: 'Karyada Irawan',
  },
  {
    id: 2,
    icon: Images.medicalProgress,
    title: 'Your Medical Record Has Been Updated',
    subtitle: 'Pertamina Hospital-Nusantara City',
    dateTime: '22 Maret 2024 - 19.54 WIB',
    physician: 'Indra Kamataru',
  },
  {
    id: 3,
    icon: Images.close,
    title: 'Your EMR has been Revoked',
    subtitle: 'Sanglah Hospital-Denpasar',
    dateTime: '21 Maret 2024 - 00.34 WIB',
    physician: 'Samika Karamoy',
  },
];

const NotificationScreen = () => {
  const renderItem = ({item}: {item: any}) => (
    <StyledView className="w-full flex flex-row items-start mb-6 space-x-4">
      <GeneralImage url={item.icon} size={24} />
      <StyledView>
        <TextPrimary text={item.title} classStyle="text-gray-800" isBold />
        <TextPrimary
          text={item.subtitle}
          classStyle="text-gray-800 text-xs my-1"
        />
        <TextPrimary text={item.dateTime} classStyle="text-gray-800 text-xs" />
      </StyledView>
    </StyledView>
  );

  return (
    <Scaffold
      topBar={
        <StyledView>
          <AppBar
            title={
              <TextPrimary
                text={Strings.label.notification}
                classStyle="text-lg text-gray-800"
                isBold={true}
              />
            }
          />
          <StyledView className="flex flex-row px-6 py-4">
            <IconLabelButton
              icon={Images.filter}
              label={Strings.label.filter}
              onPress={() => {}}
              classStyle="mr-4"
            />
            <IconLabelButton
              icon={Images.sort}
              label={Strings.label.sort}
              onPress={() => {}}
            />
          </StyledView>
        </StyledView>
      }>
      <StyledFlatListNotification
        className="w-full px-6 mt-4"
        showsVerticalScrollIndicator={false}
        data={data}
        renderItem={renderItem}
        keyExtractor={item => item.id}
      />
    </Scaffold>
  );
};

export default NotificationScreen;
