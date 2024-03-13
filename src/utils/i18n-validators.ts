import { url, required, createI18nMessage, requiredIf } from "@vuelidate/validators";
import { useI18n } from "vue-i18n";

export const useI18nValidators = () => {
  const $i18n = useI18n();
  const messagePath = ({ $validator }: { $validator: string }) =>
    `validations.${$validator}`;

  const withI18nMessage = createI18nMessage({
    t: $i18n.t.bind($i18n),
    messagePath,
  });

  return {
    required: withI18nMessage(required),
    url: withI18nMessage(url),
    requiredIf: withI18nMessage(requiredIf , { withArguments: true })
  };
};
