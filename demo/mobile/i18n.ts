import i18n from 'i18next';
import {initReactI18next} from 'react-i18next';

import global_en from './translations/en/global.json';
import global_id from './translations/id/global.json';
import * as RNLocalize from 'react-native-localize';

const resources = {
  en: {
    global: global_en,
  },
  id: {
    global: global_id,
  },
};

export const fallbackLanguage = RNLocalize.getLocales()[0].languageCode;

i18n.use(initReactI18next).init({
  resources,
  compatibilityJSON: 'v3',
  lng: fallbackLanguage,
  fallbackLng: fallbackLanguage,
  interpolation: {
    escapeValue: false,
  },
  returnObjects: true,
  react: {
    useSuspense: true,
  },
});

export default i18n;
